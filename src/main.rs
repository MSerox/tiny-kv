pub mod commands;
pub mod db;
pub mod pool;

use commands::Commands;
use db::{get_command, set_command};
use pool::PoolManager;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::sync::{Arc, RwLock};

const WORKER_COUNT: usize = 8;

fn main() {
    let pool = PoolManager::new(WORKER_COUNT);
    let db = Arc::new(RwLock::new(HashMap::new()));

    let listener = match TcpListener::bind("127.0.0.1:5302") {
        Ok(listener) => listener,
        Err(err) => {
            println!("Failed to start server... Error: {}", err);
            exit(1);
        }
    };
    println!("Server listening on port 5302...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected!");

                let db_clone = Arc::clone(&db);

                pool.execute(move || {
                    handle_connection(stream, db_clone);
                });
            }
            Err(e) => println!("Something went wrong: {}", e),
        }
    }
}

fn handle_connection(stream: TcpStream, db: Arc<RwLock<HashMap<String, String>>>) {
    let mut reader = BufReader::new(&stream);
    let mut write_stream = match stream.try_clone() {
        Ok(stream) => stream,
        Err(err) => {
            println!("Error in cloning write stream section: {}", err);
            return;
        }
    };
    let mut line = String::new();
    loop {
        line.clear();

        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("Client disconnected.");
                    break;
                }

                let line = line.trim();
                if line.is_empty() {
                    write_stream_fn(
                        &mut write_stream,
                        "Empty command received. Please enter a valid command.\n",
                    );
                    continue;
                }

                if line == "/q" {
                    write_stream_fn(&mut write_stream, "/q Entered. Exiting...");
                    break;
                }

                match Commands::from_str(line) {
                    Commands::Get { key } => {
                        match get_command(key.as_str(), &db) {
                            Some(value) => {
                                write_stream_fn(
                                    &mut write_stream,
                                    format!("Value: {}", value).as_str(),
                                );
                            }
                            None => {
                                write_stream_fn(&mut write_stream, "Key not found...\n");
                            }
                        };
                    }
                    Commands::Set { key, value } => {
                        set_command(key.as_str(), value.as_str(), &db);
                        write_stream_fn(
                            &mut write_stream,
                            format!("Set {}: {}\n", key, value).as_str(),
                        );
                    }
                    Commands::Unknown(msg) => {
                        write_stream_fn(&mut write_stream, &msg);
                    }
                }
            }
            Err(e) => {
                println!("Error reading line: {}", e);
                break;
            }
        }
    }
}

fn write_stream_fn(stream: &mut TcpStream, content: &str) {
    match stream.write_all(format!("{content}\n").as_bytes()) {
        Ok(_) => {}
        Err(err) => {
            println!("Error writing to stream: {}", err);
        }
    }
}
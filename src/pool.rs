use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, channel},
    },
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct PoolManager {
    workers: Vec<Worker>,
    producer: Option<mpsc::Sender<Job>>,
}

impl PoolManager {
    pub fn new(size: usize) -> PoolManager {
        assert!(size > 0, "Pool size must be greater than zero!");

        let (producer, consumer) = channel();
        let consumer = Arc::new(Mutex::new(consumer));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            let worker = Worker::new(i, consumer.clone());

            workers.push(worker);
        }

        PoolManager {
            workers,
            producer: Some(producer),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        match self.producer.as_ref() {
            Some(sender) => match sender.send(job) {
                Ok(_) => {}
                Err(err) => {
                    println!("Failed to send job to worker: {}", err);
                }
            },
            None => {
                println!("Thread has been shut down; cannot execute job.");
            }
        }
    }
}

impl Drop for PoolManager {
    fn drop(&mut self) {
        println!("Shutting down all workers...");
        self.producer = None;

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(handle) = worker.worker.take() {
                match handle.join() {
                    Ok(_) => {}
                    Err(err) => {
                        println!("Failed to join worker {}: {:?}", worker.id, err);
                    }
                }
            }
        }
    }
}

pub struct Worker {
    id: usize,
    worker: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let worker = thread::spawn(move || {
            loop {
                let job = match receiver.lock() {
                    Ok(guard) => guard.recv(),
                    Err(err) => {
                        println!("Worker {} failed to acquire lock: {}", id, err);
                        break;
                    }
                };

                match job {
                    Ok(job) => {
                        job();
                    }
                    Err(_) => {
                        println!("Worker {} disconnected; shutting down...", id);
                        break;
                    }
                }
            }
        });
        Worker {
            id,
            worker: Some(worker),
        }
    }
}
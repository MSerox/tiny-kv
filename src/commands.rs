pub enum Commands {
    Set { key: String, value: String },
    Get { key: String },
    Unknown(String),
}

impl Commands {
    pub fn from_str(input: &str) -> Self {
        let command_parts: Vec<&str> = input.split_whitespace().collect();
        println!("Received command: {:?}", command_parts);
        if command_parts.is_empty() {
            return Commands::Unknown("Empty command received...".to_string());
        }
        match command_parts[0].to_ascii_lowercase().as_str() {
            "set" if command_parts.len() == 3 => Commands::Set {
                key: command_parts[1].to_string(),
                value: command_parts[2].to_string(),
            },
            "get" if command_parts.len() == 2 => Commands::Get {
                key: command_parts[1].to_string(),
            },
            _ => Commands::Unknown("Unknown Input...".to_string()),
        }
    }
}
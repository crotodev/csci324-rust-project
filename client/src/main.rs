use std::env;

mod producer;
mod consumer;
mod message;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command>", args[0]);
        std::process::exit(1);
    }

    let command: &String = &args[1];
    match command.as_str() {
        "produce" => {
            if args.len() < 5 {
                eprintln!("Usage: {} <address> <topic> <n>", args[0]);
                std::process::exit(1);
            }
            let address: &String = &args[2];
            let topic: &String = &args[3];
            let n: &i32 = &args[4].parse::<i32>().unwrap();

            let mut p: producer::Producer = producer::Producer::new(address).unwrap();
            
            for _ in 0..*n {
                if let Err(e) = p.send(topic, "Hello, world!") {
                    eprintln!("Failed to send message: {}", e);
                }
            }
        }
        "consume" => {
            if args.len() < 4 {
                eprintln!("Usage: {} <address> <topic>", args[0]);
                std::process::exit(1);
            }

            let address: &String = &args[2];
            let topic: &String = &args[3];
            
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}

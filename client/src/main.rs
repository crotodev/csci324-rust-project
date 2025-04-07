use std::env;

mod consumer;
mod message;
mod producer;

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
            let n: &i64 = &args[4].parse::<i64>().unwrap();
            let mut p: producer::Producer = producer::Producer::new(address).unwrap();

            for _ in 0..*n {
                if let Err(e) = p.send(topic, "Hello, world!") {
                    eprintln!("Failed to send message: {}", e);
                }
            }
        }
        "consume" => {
            if args.len() < 4 {
                eprintln!(
                    "Usage: {} consume <address> <topic1> [<topic2> ...]",
                    args[0]
                );
                std::process::exit(1);
            }

            let address: &String = &args[2];
            let topics: Vec<&str> = args[3..].iter().map(|s| s.as_str()).collect();

            let mut c: consumer::Consumer = consumer::Consumer::new(address).unwrap();
            c.add_topics(&topics);
            println!("Consumer with address: {}, topics: {:?}", address, topics);
            loop {
                c.poll().unwrap();
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}

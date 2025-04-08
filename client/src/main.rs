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
            let (address, topic, n) = if args.len() >= 3 && args[2] == "test" {
                let n = if args.len() == 4 {
                    args[3].parse::<i64>().unwrap_or(100)
                } else {
                    100
                };
                ("127.0.0.1:9092".to_string(), "test-topic".to_string(), n)
            } else if args.len() < 5 {
                eprintln!("Usage: {} produce <address> <topic> <n> or {} produce test [n]", args[0], args[0]);
                std::process::exit(1);
            } else {
                (args[2].clone(), args[3].clone(), args[4].parse::<i64>().unwrap())
            };

            let mut p: producer::Producer = producer::Producer::new(&address).unwrap();

            for _ in 0..n {
                if let Err(e) = p.send(&topic, "Hello, world!") {
                    eprintln!("Failed to send message: {}", e);
                }
            }
            println!("Produced {} messages to topic '{}'", n, topic);
        }
        "consume" => {
            let (address, topics) = if args.len() == 3 && args[2] == "test" {
                ("127.0.0.1:9092".to_string(), vec!["test-topic"])
            } else if args.len() < 4 {
                eprintln!(
                    "Usage: {} consume <address> <topic1> [<topic2> ...] or {} consume test",
                    args[0], args[0]
                );
                std::process::exit(1);
            } else {
                (args[2].clone(), args[3..].iter().map(|s| s.as_str()).collect())
            };

            let mut c: consumer::Consumer = consumer::Consumer::new(&address).unwrap();
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

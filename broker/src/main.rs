mod broker;
mod topic;

use broker::start_broker;
use std::env;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let default_address = "127.0.0.1:9092".to_string();
    let address = args.get(1).unwrap_or(&default_address);

    start_broker(address);
}

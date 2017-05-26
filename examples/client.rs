extern crate rand;
extern crate steamstreaming;

use std::env;

fn main() {
    let client_id = rand::random::<u64>();
    let hostname = env::args().nth(1).unwrap();
    let psk = env::args().nth(2).unwrap();
    steamstreaming::connect((hostname.as_str(), steamstreaming::CONTROL_PORT), &psk, client_id);
}

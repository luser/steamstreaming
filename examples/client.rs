extern crate rand;
extern crate steamstreaming;

use std::env;
const CONTROL_PORT: u16 = 27036;

fn main() {
    let client_id = rand::random::<u64>();
    let hostname = env::args().nth(1).unwrap();
    let psk = env::args().nth(2).unwrap();
    steamstreaming::connect((hostname.as_str(), CONTROL_PORT), &psk, client_id);
}

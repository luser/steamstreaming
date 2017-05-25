extern crate rand;
extern crate steamstreaming;

fn main() {
    let client_id = rand::random::<u64>();
    steamstreaming::discover(client_id, 0).unwrap();
}

extern crate rustc_serialize;
extern crate steamstreaming;

use rustc_serialize::hex::FromHex;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use steamstreaming::crypto;
use steamstreaming::errors::*;

fn run() -> Result<()> {
    let mut in_file = BufReader::new(File::open(env::args_os().nth(1).expect("No input file"))?);
    let key = env::args().nth(2).expect("No private key").from_hex()?;
    let mut out_file = File::create(env::args_os().nth(3).expect("No output file"))?;
    let mut data = vec!();
    in_file.read_to_end(&mut data)?;
    let decrypted = crypto::symmetric_decrypt(&data, &key)?;
    out_file.write_all(&decrypted)?;
    Ok(())
}

fn main() {
    run().unwrap();
}

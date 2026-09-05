use std::fs;
use std::io;
use std::env::args;

fn main() {
    let directory: Vec<String> = args().collect();
    println!("{:?}", directory);
}

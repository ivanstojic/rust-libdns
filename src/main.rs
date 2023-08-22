use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("./example.dat");

    match contents {
        Ok(payload) => {
            println!("data: {payload}");
        }
        Err(_) => {
            println!("well shucks!");
        }
    }

}

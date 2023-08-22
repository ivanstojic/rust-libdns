mod dns;

use std::fs;
use crate::dns::data::{DNSClass, DNSType, make_request};

fn main() {
    println!("Hello, world!");

    let contents = fs::read("./test-data/response-photos.ivanstojic.com.raw");

    match contents {
        Ok(payload) => {
            let packet = make_request(String::from("photos.ivanstojic.com"), DNSType::A, DNSClass::IN);
            println!("data!");
        }
        Err(_) => {
            println!("well shucks!");
        }
    }

}

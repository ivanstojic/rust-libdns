mod dns;

use std::fs;
use crate::dns::data::{DNSClass, DNSType, make_request, read_response, Response};

fn main() {
    println!("Hello, world!");

    let contents = fs::read("./test-data/response-photos.ivanstojic.com.raw");

    match contents {
        Ok(payload) => {
            let packet = make_request(String::from("photos.ivanstojic.com"), DNSClass::IN, DNSType::A);
            let resp = read_response(payload);

            match resp {
                Ok(_) => { println!("works ok"); }
                Err(e) => { println!("failed: {}", e); }
            }
        }
        Err(_) => {
            println!("well shucks!");
        }
    }

}

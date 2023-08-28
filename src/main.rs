mod dns;

use crate::dns::data::*;
use std::fs;



fn main() {
    let contents = fs::read("./test-data/response-photos.ivanstojic.com.raw");

    match contents {
        Ok(payload) => {
            let _packet = make_packet(
                String::from("photos.ivanstojic.com"),
                DNSClass::IN,
                DNSType::A,
            );
            let resp = read_packet(payload);

            match resp {
                Ok(p) => {
                    println!("response:\n{}", p);
                }
                Err(e) => {
                    println!("parsing failed: {}", e);
                }
            }
        }
        Err(_) => {
            println!("well shucks!");
        }
    }
}

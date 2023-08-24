use std::fmt::Formatter;

pub enum RequestResponse {
    Request,
    Response,
}

pub enum DNSType {
    A = 1,
}

pub enum DNSClass {
    IN = 1,
}

pub struct Header {
    rr_flag: RequestResponse,
    opcode: u8,
    authoritative: bool,
    truncated: bool,
    recursion_desired: bool,
    recursion_available: bool,
    authenticated: bool,
    non_authenticated_acceptable: bool,
    reply_code: u8,
}

pub struct Request {
    header: Header
}

pub struct Response {

}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str_req = match self.header.rr_flag {
            RequestResponse::Request => "QUERY",
            RequestResponse::Response => "ANSWER"
        };

        writeln!(f, ";; ->>HEADER<<- opcode: {}, status: ???, id: ???", str_req);
        writeln!(f, ";; flags: ???; QUERY: 1, ANSWER: 2, AUTHORITY:3, ADDITIONAL: 4")
    }
}

pub fn make_request(_name: String, _c: DNSClass, _t: DNSType) -> Request {
    return Request {
        header: Header {
            rr_flag: RequestResponse::Request,
            opcode: 1,
            authoritative: false,
            truncated: false,
            recursion_desired: false,
            recursion_available: false,
            authenticated: false,
            non_authenticated_acceptable: false,
            reply_code: 0
        }
    }
}

pub fn read_response(d: Vec<u8>) -> Result<Response, &'static str> {
    if d.len() < 53 {
        return Err("this is broken");
    } else {
        return Ok(Response{})
    }
}

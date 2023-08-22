use std;

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

pub struct PacketHeader {
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

pub struct RequestPacket {
    header: PacketHeader
}

pub fn make_request(name: String, t: DNSType, c: DNSClass) -> RequestPacket {
    return RequestPacket {
        header: PacketHeader {
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

use std::fmt::{Display, Formatter};
use std::string::String;

pub enum RequestResponse {
    Request,
    Response,
}

#[allow(dead_code)]
pub enum DNSType {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    MX = 15,
    TXT = 16,
}

impl TryFrom<u16> for DNSType {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DNSType::A),
            _ => Err(())
        }
    }
}

impl Display for DNSType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DNSType::A => write!(f, "A"),
            DNSType::NS => write!(f, "NS"),
            DNSType::CNAME => write!(f, "CNAME"),
            DNSType::SOA => write!(f, "SOA"),
            DNSType::MX => write!(f, "MX"),
            DNSType::TXT => write!(f, "TXT"),
        }
    }
}

pub enum DNSClass {
    IN = 1,
}

impl TryFrom<u16> for DNSClass {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DNSClass::IN),
            _ => Err(())
        }
    }
}

impl Display for DNSClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { DNSClass::IN => write!(f, "IN") }
    }
}

pub struct Header {
    rr_flag: RequestResponse,
    opcode: u8,
    authoritative: bool,
    truncated: bool,
    recursion_desired: bool,
    recursion_available: bool,
    authenticated_data: bool,
    checking_disabled: bool,
    reply_code: u8,
}

pub struct Question {
    name: String,
    qtype: u16,
    qclass: u16
}

impl Display for Question {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, ";{:32} {:8} {:8}", &self.name,
                 DNSClass::try_from(self.qclass).expect("?"),
                 DNSType::try_from(self.qtype).expect("?"))
    }
}

pub struct Packet {
    transaction_id: u16,
    header: Header,
    questions: Vec<Question>,
    answers: Vec<Question>,
    authority: Vec<Question>,
    additional: Vec<Question>,
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str_req = match self.header.rr_flag {
            RequestResponse::Request => "QUERY",
            RequestResponse::Response => "ANSWER",
        };

        let mut flags: String = "".to_owned();

        if self.header.authoritative {
            flags.push_str(" aa");
        }
        if self.header.truncated {
            flags.push_str(" tc");
        }
        if self.header.recursion_desired {
            flags.push_str(" rd");
        }
        if self.header.recursion_available {
            flags.push_str(" ra");
        }
        if self.header.authenticated_data {
            flags.push_str(" ad");
        }
        if self.header.checking_disabled {
            flags.push_str(" cd");
        }

        writeln!(
            f,
            ";; ->>HEADER<<- opcode: {}, status: {}, id: {:x}, {}",
            str_req, self.header.opcode, self.transaction_id, self.header.reply_code
        )?;

        writeln!(
            f,
            ";; flags:{} QUERY: {}, ANSWER: {}, AUTHORITY: {}, ADDITIONAL: {}",
            flags,
            self.questions.len(),
            self.answers.len(),
            self.authority.len(),
            self.additional.len()
        )?;

        writeln!(f, "\n;; QUESTION SECTION:")?;
        for question in self.questions.iter() {
            write!(f, "{}", question)?;
        }

        Ok(())
    }
}

pub fn make_packet(_name: String, _c: DNSClass, _t: DNSType) -> Packet {
    return Packet {
        transaction_id: 0,
        header: Header {
            rr_flag: RequestResponse::Request,
            opcode: 1,
            authoritative: false,
            truncated: false,
            recursion_desired: false,
            recursion_available: false,
            authenticated_data: false,
            checking_disabled: false,
            reply_code: 0,
        },
        questions: Vec::with_capacity(0),
        answers: Vec::with_capacity(0),
        authority: Vec::with_capacity(0),
        additional: Vec::with_capacity(0),
    };
}

pub fn read_name(buf: &[u8]) -> (String, usize) {
    let mut result = "".to_owned();
    let mut idx = 0;

    loop {
        let len = buf[idx] as usize;
        idx += 1;

        if len == 0 {
            break;
        }

        let name = String::from_utf8(buf[idx..idx + len].to_vec());

        match name {
            Ok(namestr) => {
                result.push_str(namestr.as_str());
                result.push_str(".");
                idx += len;
            }
            Err(_) => {
                break;
            }
        }
    }

    return (result, idx);
}

pub fn read_packet(d: Vec<u8>) -> Result<Packet, String> {
    return if d.len() < 53 {
        Err(format!("not enough bytes, only found {}", d.len()))
    } else {
        let transaction_id = ((d[0] as u16) << 8) | (d[1] as u16);

        // header
        let header = Header {
            rr_flag: if d[2] & 0b1000_0000 != 0 {
                RequestResponse::Request
            } else {
                RequestResponse::Response
            },
            opcode: d[2] & 0b0111_1000 >> 3,
            authoritative: d[2] & 0b0000_0100 != 0,
            truncated: d[2] & 0b0000_0010 != 0,
            recursion_desired: d[2] & 0b0000_0001 != 0,
            recursion_available: d[3] & 0b1000_0000 != 0,
            authenticated_data: d[3] & 0b0010_0000 != 0,
            checking_disabled: d[3] & 0b0001_0000 != 0,
            reply_code: d[3] & 0b0000_1111,
        };

        let qdcount = ((d[4] as u16) << 8) | (d[5] as u16);
        let mut queries = Vec::with_capacity(qdcount as usize);

        let ancount = ((d[6] as u16) << 8) | (d[7] as u16);
        let nscount = ((d[8] as u16) << 8) | (d[9] as u16);
        let arcount = ((d[10] as u16) << 8) | (d[11] as u16);

        let sub = &d[12..];
        match read_name(sub) {
            (name, nextidx) => {
                println!("name: {}, next idx: {}", name, nextidx);
                let qtype = ((sub[nextidx] as u16) << 8) | (sub[nextidx+1] as u16);
                let qclass = ((sub[nextidx+2] as u16) << 8) | (sub[nextidx+3] as u16);
                println!("type {} class {}", qtype, qclass);
                queries.push(Question {
                    name,
                    qtype,
                    qclass
                });
            }
        }

        Ok(Packet {
            transaction_id,
            header,
            questions: queries,
            answers: Vec::with_capacity(ancount as usize),
            authority: Vec::with_capacity(nscount as usize),
            additional: Vec::with_capacity(arcount as usize),
        })
    };
}

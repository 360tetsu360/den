use std::io::Read;

use byteorder::{ReadBytesExt, BigEndian};
use bytes::{BufMut, BytesMut};
use den::*;

pub const MAGIC: [u8; 16] = [
    0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78,
];

struct Magic;

impl DenWith<bool> for Magic {
    fn decode(bytes: &mut std::io::Cursor<&[u8]>) -> std::io::Result<bool> {
        let mut magic = [0; 16];
        bytes.read_exact(&mut magic)?;
        Ok(magic == MAGIC)
    }

    fn encode(_v: &bool, bytes: &mut bytes::BytesMut) {
        bytes.put_slice(&MAGIC as &[u8]);
    }

    fn size(_v: &bool) -> usize {
        16
    }
}

struct StrDen;

impl DenWith<String> for StrDen {
    fn decode(bytes: &mut std::io::Cursor<&[u8]>) -> std::io::Result<String> {
        let len = bytes.read_u16::<BigEndian>()?;
        let mut str_dist = vec![0u8;len as usize];
        bytes.read_exact(&mut str_dist)?;
        Ok(String::from_utf8(str_dist).unwrap())
    }

    fn encode(v: &String, bytes: &mut BytesMut) {
        bytes.put_u16(v.len() as u16);
        bytes.put_slice(v.as_bytes())
    }

    fn size(v: &String) -> usize {
        2 + v.len()
    }
}

#[derive(Den)]
struct Ping {
    #[den(with = "Big")]
    time : i64,
    #[den(with = "Magic")]
    _magic : bool,
    #[den(with = "Big")]
    guid: u64,
    #[den(with = "StrDen")]
    message : String
}

fn main() {
    let ping = Ping{ time: 0, _magic: true, guid: 0x00111,message : "Hello World".to_string() };
    let mut bytes = BytesMut::new();
    ping.encode(&mut bytes);
    dbg!(bytes);
}
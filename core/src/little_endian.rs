use crate::DenWith;
use byteorder::{LittleEndian, ReadBytesExt};
use bytes::BufMut;
use std::io::{Cursor, Result};

pub struct Little;

impl DenWith<u16> for Little {
    fn decode(bytes: &mut Cursor<&[u8]>) -> Result<u16> {
        bytes.read_u16::<LittleEndian>()
    }

    fn encode(v: &u16, bytes: &mut bytes::BytesMut) {
        bytes.put_u16_le(*v)
    }

    fn size(_: &u16) -> usize {
        2
    }
}

impl DenWith<i16> for Little {
    fn decode(bytes: &mut Cursor<&[u8]>) -> Result<i16> {
        bytes.read_i16::<LittleEndian>()
    }

    fn encode(v: &i16, bytes: &mut bytes::BytesMut) {
        bytes.put_i16_le(*v)
    }

    fn size(_: &i16) -> usize {
        2
    }
}

impl DenWith<u32> for Little {
    fn decode(bytes: &mut Cursor<&[u8]>) -> Result<u32> {
        bytes.read_u32::<LittleEndian>()
    }

    fn encode(v: &u32, bytes: &mut bytes::BytesMut) {
        bytes.put_u32_le(*v)
    }

    fn size(_: &u32) -> usize {
        4
    }
}

impl DenWith<i32> for Little {
    fn decode(bytes: &mut Cursor<&[u8]>) -> Result<i32> {
        bytes.read_i32::<LittleEndian>()
    }

    fn encode(v: &i32, bytes: &mut bytes::BytesMut) {
        bytes.put_i32_le(*v)
    }

    fn size(_: &i32) -> usize {
        4
    }
}

impl DenWith<u64> for Little {
    fn decode(bytes: &mut Cursor<&[u8]>) -> Result<u64> {
        bytes.read_u64::<LittleEndian>()
    }

    fn encode(v: &u64, bytes: &mut bytes::BytesMut) {
        bytes.put_u64_le(*v)
    }

    fn size(_: &u64) -> usize {
        8
    }
}

impl DenWith<i64> for Little {
    fn decode(bytes: &mut Cursor<&[u8]>) -> Result<i64> {
        bytes.read_i64::<LittleEndian>()
    }

    fn encode(v: &i64, bytes: &mut bytes::BytesMut) {
        bytes.put_i64_le(*v)
    }

    fn size(_: &i64) -> usize {
        8
    }
}

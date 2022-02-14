use crate::DenWith;
use minecraft_varint::*;
use std::io::{Cursor, Result};

pub struct Var;

impl DenWith<u32> for Var {
    fn decode(bytes: &mut Cursor<&[u8]>) -> Result<u32> {
        bytes.read_var_u32()
    }

    fn encode(v: &u32, bytes: &mut bytes::BytesMut) {
        bytes.write_var_u32(*v);
    }

    fn size(v: &u32) -> usize {
        var_u32_len(*v)
    }
}

impl DenWith<i32> for Var {
    fn decode(bytes: &mut Cursor<&[u8]>) -> Result<i32> {
        bytes.read_var_i32()
    }

    fn encode(v: &i32, bytes: &mut bytes::BytesMut) {
        bytes.write_var_i32(*v);
    }

    fn size(v: &i32) -> usize {
        var_i32_len(*v)
    }
}

impl DenWith<u64> for Var {
    fn decode(bytes: &mut Cursor<&[u8]>) -> Result<u64> {
        bytes.read_var_u64()
    }

    fn encode(v: &u64, bytes: &mut bytes::BytesMut) {
        bytes.write_var_u64(*v);
    }

    fn size(v: &u64) -> usize {
        var_u64_len(*v)
    }
}

impl DenWith<i64> for Var {
    fn decode(bytes: &mut Cursor<&[u8]>) -> Result<i64> {
        bytes.read_var_i64()
    }

    fn encode(v: &i64, bytes: &mut bytes::BytesMut) {
        bytes.write_var_i64(*v);
    }

    fn size(v: &i64) -> usize {
        var_i64_len(*v)
    }
}
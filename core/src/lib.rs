use byteorder::ReadBytesExt;
use bytes::BufMut;
use bytes::BytesMut;
use std::io::{Cursor, Result};
pub(crate) mod big_endian;
pub(crate) mod little_endian;
pub(crate) mod var;
pub(crate) mod u24;
pub use big_endian::Big;
pub use little_endian::*;
pub use var::Var;
pub use u24::U24;

pub trait Den {
    fn decode(bytes: &mut Cursor<&[u8]>) -> Result<Self>
    where
        Self: Sized;
    fn encode(&self, bytes: &mut BytesMut);
    fn size(&self) -> usize;
}

pub trait DenWith<T> {
    fn decode(bytes: &mut Cursor<&[u8]>) -> Result<T>
    where
        T: Sized;
    fn encode(v: &T, bytes: &mut BytesMut);
    fn size(v: &T) -> usize;
}

impl Den for u8 {
    fn decode(bytes: &mut Cursor<&[u8]>) -> Result<Self> {
        bytes.read_u8()
    }

    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_u8(*self)
    }

    fn size(&self) -> usize {
        1
    }
}

impl Den for i8 {
    fn decode(bytes: &mut Cursor<&[u8]>) -> Result<Self> {
        bytes.read_i8()
    }

    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_i8(*self)
    }

    fn size(&self) -> usize {
        1
    }
}

impl Den for bool {
    fn decode(bytes: &mut Cursor<&[u8]>) -> Result<Self>{
        Ok(bytes.read_u8()? != 0)
    }

    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_u8(*self as u8)
    }

    fn size(&self) -> usize {
        1
    }
}
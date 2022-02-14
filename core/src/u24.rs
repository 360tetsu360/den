use byteorder::{ReadBytesExt, LittleEndian, ByteOrder};
use bytes::BufMut;

use crate::DenWith;

pub struct U24;

impl DenWith<u32> for U24 {
    fn decode(bytes: &mut std::io::Cursor<&[u8]>) -> std::io::Result<u32> {
        bytes.read_u24::<LittleEndian>()
    }

    fn encode(v: &u32, bytes: &mut bytes::BytesMut) {
        let mut buf = [0; 3];
        LittleEndian::write_u24(&mut buf, *v);
        bytes.put(&buf as &[u8]);
    }

    fn size(_: &u32) -> usize {
        3
    }
}
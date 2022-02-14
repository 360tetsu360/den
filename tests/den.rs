use bytes::BytesMut;
use den::*;

#[derive(Debug,PartialEq,Den)]
struct Hoge {
    u8 : u8,
    i8 : i8,
    #[den(with = "Big")]
    u16 : u16,
    u16_le : u16,
    #[den(with = "Big")]
    i16 : i16,
    i16_le : i16,
    #[den(with = "Big")]
    u32 : u32,
    u32_le : u32,
    #[den(with = "Big")]
    i32 : i32,
    i32_le : i32,
    #[den(with = "Big")]
    u64 : u64,
    u64_le : u64,
    #[den(with = "Big")]
    i64 : i64,
    i64_le : i64,
    bool : bool,
    #[den(with = "U24")]
    u24 : u32,
    #[den(with = "Var")]
    var_u32 : u32,
    #[den(with = "Var")]
    var_i32 : i32,
    #[den(with = "Var")]
    var_u64 : u64,
    #[den(with = "Var")]
    var_i64 : i64,
}

#[test]
fn den() {
    let hoge = Hoge {
        u8: 0x0F,
        i8: -0x80,
        u16: 0xF000,
        u16_le: 0x000F,
        i16: -0x00F0,
        i16_le: -0x0F00,
        u32: 0xF0000000,
        u32_le: 0x0000000F,
        i32: -0x80000000,
        i32_le: -0x00000008,
        u64: 0xF,
        u64_le: 0xF00000000000000,
        i64: -0x8,
        i64_le: -0x800000000000000,
        bool: false,
        u24 : 0xF00000,
        var_u32: 0x1000,
        var_i32: -0x2100,
        var_u64: 0x2000000,
        var_i64: -0x20000000,
    };

    let mut bytes = BytesMut::new();
    hoge.encode(&mut bytes);
    assert_eq!(hoge.size(),bytes.len());

    let mut cursor = std::io::Cursor::new(&bytes as &[u8]);
    let hoge2 = Hoge::decode(&mut cursor).unwrap();
    assert_eq!(hoge,hoge2)
}
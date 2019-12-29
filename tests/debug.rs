extern crate bitpayload;
use bitpayload::*;

#[test]
fn fmt_dbg_enum_value() {

    use IntType::*;

    assert_eq!("u8", format!("{:?}", UInt8));
    assert_eq!("u16", format!("{:?}", UInt16));
    assert_eq!("u32", format!("{:?}", UInt32));
    assert_eq!("u64", format!("{:?}", UInt64));
    assert_eq!("i8", format!("{:?}", SInt8));
    assert_eq!("i16", format!("{:?}", SInt16));
    assert_eq!("i32", format!("{:?}", SInt32));
    assert_eq!("i64", format!("{:?}", SInt64));

}
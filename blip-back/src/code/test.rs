use crate::code::*;

#[test]
fn simple_high_low() {
    assert_eq!(high_low(0u8), (0u8, 0u8));
    assert_eq!(high_low(0xff), (0xf, 0xf));
    assert_eq!(high_low(0xbc), (0xb, 0xc));
    assert_eq!(high_low(0x0f), (0x0, 0xf));
    assert_eq!(high_low(0xaf), (0xa, 0xf));
}
#[test]
fn byte_to_hex_simple() {
    assert_eq!(byte_to_hex_string(0x00, 0), "aa");
    assert_eq!(byte_to_hex_string(0x01, 0), "ab");
    assert_eq!(byte_to_hex_string(0x00, 1), "bb");
    assert_eq!(byte_to_hex_string(0xff, 0), "pp");
    assert_eq!(byte_to_hex_string(0xff, 9), "yy");
    assert_eq!(byte_to_hex_string(0xff, 10), "zz");
}
#[test]
fn string() {
    assert_eq!(hash("hello world",0), "folgdllloabooonajdmlccllipfkmnmd")
}

#[test]
fn mod11() {
    assert_eq!(1649249019186u64 % 11, 10);
    assert_eq!(1649249019187u64 % 11, 0);
    assert_eq!(1649249019188u64 % 11, 1);
}

#[test]
#[ignore]
fn dates(){
    use chrono::{DateTime, Utc};

    let now: DateTime<Utc> = Utc::now();

    println!("UTC now is: {}", now);
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    println!("UTC now in a custom format is: {}", now.format("%a %b %e %T %Y"));
    println!("timestamp is {}",now.timestamp_millis());
    println!("is {} mins later than birth.",(1649251893631 - 1649249019186u64)/1000/60);
    panic!("log")
}
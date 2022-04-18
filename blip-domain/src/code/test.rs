use crate::code::*;



#[test]
fn mod11() {
    assert_eq!(1649249019186u64 % 11, 10);
    assert_eq!(1649249019187u64 % 11, 0);
    assert_eq!(1649249019188u64 % 11, 1);
}

// #[test]
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

#[test]
fn simple_digits(){
    
    assert_eq!(digits(12345,10),vec![5,4,3,2,1]);
}
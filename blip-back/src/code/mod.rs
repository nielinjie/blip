pub fn byte_to_hex_string(b: u8, offset: u8) -> String {
    let (high, low) = high_low(b);
    let array: Vec<char> = ('a'..='z').into_iter().collect();
    let high_string = array.get((high as usize) + (offset as usize)).unwrap();
    let low_string = array.get((low as usize) + (offset as usize)).unwrap();
    format!("{}{}", high_string, low_string)
}
pub fn hash(s: &str,offset: u8) -> String {
    let md: [u8; 16] = md5::compute(s.as_bytes()).into();
    let array: Vec<String> = md.into_iter().map(|x| byte_to_hex_string(x, offset)).collect();
    array.join("").to_string()
}

fn high_low(b: u8) -> (u8, u8) {
    let high = b.checked_shr(4).unwrap();
    let low = b - high * 16;
    (high, low)
}

#[cfg(test)]
mod test;

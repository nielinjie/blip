pub fn hash(s: &str) -> Vec<char> {
    let md: [u8; 16] = md5::compute(s.as_bytes()).into();
    let number: u128 = u128::from_be_bytes(md);
    let array: [char; 26] = ('a'..='z').collect::<Vec<char>>().try_into().unwrap();
    let digits = digits(number, 26);
    let re: Vec<char> = digits.into_iter().map(|d| array[d as usize]).collect();
    re
}

pub fn digits(number: u128, base: u128) -> Vec<u8> {
    let mut n = number;
    let mut re = Vec::new();
    while n > 0 {
        let yu = n % base;
        n = n / base;
        re.push(yu as u8);
    }
    // re.into_iter().cycle().take(length).collect()
    re
}

#[cfg(test)]
mod test;

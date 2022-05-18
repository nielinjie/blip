pub fn hash(s: &str) -> Vec<char> {
    let md: [u8; 16] = md5::compute(s.as_bytes()).into();
    let number: u128 = u128::from_be_bytes(md);
    let charset: Vec<char> = ('a'..='z').collect::<Vec<char>>();
    let digits = digits(number, (&charset).len());
    let re: Vec<char> = digits.into_iter().map(|d| charset[d as usize]).collect();
    re
}

pub fn digits(number: u128, base: usize) -> Vec<u8> {
    let mut n = number;
    let b = base as u128;
    let mut re = Vec::new();
    while n > 0 {
        let yu = n % b;
        n = n / b;
        re.push(yu as u8);
    }
    // re.into_iter().cycle().take(length).collect()
    re
}

#[cfg(test)]
mod test;

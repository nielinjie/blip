use regex::Regex;

pub fn mask_available(mask: &str) -> Result<String, String> {
    let available_mask_regex = r"^\*?[a-z]+\*?$";
    let available = Regex::new(available_mask_regex).unwrap();
    if available.find(mask).is_none() {
        return Err(format!("available mask is {}", available_mask_regex));
    }
    Ok(mask.to_string())
}
fn mask_to_regex(mask: &str) -> Result<String, String> {
    mask_available(mask)?;
    let mut re = "".to_string();
    if mask.starts_with("*") {
        if mask.ends_with("*") {
            re.push_str(".+?(");
            re.push_str(&mask[1..mask.len() - 1]);
            re.push_str(").+");
        } else {
            re.push_str(".+?(");
            re.push_str(&mask[1..mask.len()]);
            re.push_str(")");
        }
    } else {
        if mask.ends_with("*") {
            re.push_str("(");
            re.push_str(&mask[0..mask.len() - 1]);
            re.push_str(").+");
        } else {
            re.push_str("(");
            re.push_str(&mask[0..mask.len()]);
            re.push_str(")");
        }
    }
    //convert glob style to regex style
    let mut res = "^".to_string();
    res.push_str(&re);
    res.push_str("$");
    Ok(res)
}
fn match_(code: &str, mask_regex_str: &str) -> Option<(usize, usize)> {
    // let re = mask_to_regex(mask);
    let reg = Regex::new(mask_regex_str).unwrap();
    let mat = reg.find(code);
    mat.map(|m| (m.start(), m.end()))
}
fn match_captures(code: &str, mask_regex_str: &str) -> Option<(usize, usize)> {
    let reg = Regex::new(mask_regex_str).unwrap();
    let mat = reg.captures(code);
    mat.map(|c| c.get(1).map(|m| (m.start(), m.end())))
        .flatten()
}

pub fn match_on_code(code: &str, mask: &str) -> Option<(usize, usize)> {
    let regex_string = mask_to_regex(mask);
    match_captures(code, &regex_string.unwrap())
}

#[cfg(test)]
#[test]
fn good_match() {}

#[test]
fn regex_practice() {
    let mask = "^a$";
    let re = match_("abc", mask);
    assert!(re.is_none());
    let mask = "^a.+$";
    let re = match_("abc", mask);
    assert_eq!(re, Some((0, 3)));
    let mask = "^(a).+$";
    let re = match_captures("abc", mask);
    assert_eq!(re, Some((0, 1)));
}
#[test]
fn mask_to_regex_test() {
    let mask = "a";
    let re = mask_to_regex(mask);
    assert_eq!(re, Ok("^(a)$".to_string()));
    let mask = "a*";
    let re = mask_to_regex(mask);
    assert_eq!(re, Ok("^(a).+$".to_string()));
    let mask = "*a";
    let re = mask_to_regex(mask);
    assert_eq!(re, Ok("^.+?(a)$".to_string()));
    let mask = "*a*";
    let re = mask_to_regex(mask);
    assert_eq!(re, Ok("^.+?(a).+$".to_string()));

    let mask = "*";
    let re = mask_to_regex(mask);
    assert!(re.is_err());
    let mask = "*a*b";
    let re = mask_to_regex(mask);
    assert!(re.is_err());
    let mask = "**a";
    let re = mask_to_regex(mask);
    assert!(re.is_err());
    let mask = "a**";
    let re = mask_to_regex(mask);
    assert!(re.is_err());
}
#[test]
fn match_finally() {
    let mask = "a*";
    let re = match_on_code("abc", mask);
    assert_eq!(re, Some((0, 1)));
    let mask = "a";
    let re = match_on_code("abc", mask);
    assert_eq!(re, None);

    let mask = "*a*";
    let re = match_on_code("babc", mask);
    assert_eq!(re, Some((1, 2)));
    let mask = "*a*";
    let re = match_on_code("babac", mask);
    assert_eq!(re, Some((1, 2)));
    let mask = "*a";
    let re = match_on_code("babc", mask);
    assert_eq!(re, None);
    let mask = "*a";
    let re = match_on_code("fsba", mask);
    assert_eq!(re, Some((3, 4)));
     let mask = "*ba";
    let re = match_on_code("fsba", mask);
    assert_eq!(re, Some((2, 4)));
    let mask = "*ab*";
    let re = match_on_code("babac", mask);
    assert_eq!(re, Some((1, 3)));
}

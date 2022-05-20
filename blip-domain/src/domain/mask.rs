use nom::bytes::complete::tag;
use nom::combinator::{complete, eof};
use nom::sequence::terminated;
use nom::Finish;
use nom::{
    character::complete::alpha1,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use regex::{Match, Regex};

use crate::domain::piece::Pieces;
#[derive(Debug, PartialEq, Clone)]
pub struct Mask(bool, Vec<String>, bool);
#[derive(Debug, PartialEq, Clone)]
pub enum MaskResult {
    NoMatch,
    Matched(Vec<(usize, usize)>),
}

impl Mask {
    fn to_regex_str(&self) -> String {
        let mut ret = String::new();
        if self.0 {
            ret.push_str(".+?");
        }
        let middle = self
            .1
            .iter()
            .map(|s| format!("({})", s))
            .collect::<Vec<String>>()
            .join(".+?");
        ret.push_str(&middle);
        if self.2 {
            ret.push_str(".+")
        }

        format!("^{}$", ret)
    }
    pub fn apply_to_code(&self, code: &str) -> Result<MaskResult, String> {
        let r = Regex::new(&self.to_regex_str());
        match r {
            Err(e) => return Err(format!("{}", e)),
            Ok(r) => {
                let mats = r
                    .captures(code)
                    .map(|c| c.iter().collect::<Option<Vec<Match>>>()); //.flatten();
                match mats {
                    Some(Some(m)) => Ok(if m.len() != 0 {
                        MaskResult::Matched(
                            m[1..m.len()]
                                .into_iter()
                                .map(|m| (m.start(), m.end()))
                                .collect(),
                        )
                    } else {
                        MaskResult::NoMatch
                    }),
                    _ => Ok(MaskResult::NoMatch),
                }
            }
        }
    }
    pub fn from_str(input: &str) -> Result<Self, String> {
        str_to_mask(input)
    }
}
fn middle_parser(input: &str) -> IResult<&str, Vec<String>> {
    map(
        complete(separated_list1(tag("-"), alpha1)),
        |v: Vec<&str>| {
            v.into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        },
    )(input)
}
fn mask_parser(input: &str) -> IResult<&str, Mask> {
    map(
        terminated(tuple((opt(tag("-")), middle_parser, opt(tag("-")))), eof),
        |(first, v, last)| Mask(first.is_some(), v, last.is_some()),
    )(input)
}
fn str_to_mask(input: &str) -> Result<Mask, String> {
    match mask_parser(input).finish() {
        Ok((_input, mask)) => Ok(mask),
        Err(err) => Err(format!("{}", err)),
    }
}

#[cfg(test)]
#[test]
fn into_regex_str() {
    let mask = Mask(false, vec!["a".to_string()], true);
    assert_eq!(mask.to_regex_str(), "^(a).+$");
    let mask = Mask(true, vec!["a".to_string()], true);
    assert_eq!(mask.to_regex_str(), "^.+?(a).+$");
    let mask = Mask(true, vec!["a".to_string(), "bc".to_string()], true);
    assert_eq!(mask.to_regex_str(), "^.+?(a).+?(bc).+$");
    let mask = Mask(false, vec!["a".to_string(), "bc".to_string()], true);
    assert_eq!(mask.to_regex_str(), "^(a).+?(bc).+$");
    let mask = Mask(true, vec!["a".to_string(), "bc".to_string()], false);
    assert_eq!(mask.to_regex_str(), "^.+?(a).+?(bc)$");
    let mask = Mask(false, vec!["a".to_string(), "bc".to_string()], false);
    assert_eq!(mask.to_regex_str(), "^(a).+?(bc)$");
}

#[test]
fn mask_parse() {
    let re = str_to_mask("a");
    assert_eq!(re, Ok(Mask(false, vec!["a".to_string()], false)));
    let re = str_to_mask("-a");
    assert_eq!(re, Ok(Mask(true, vec!["a".to_string()], false)));
    let re = str_to_mask("-a-");
    assert_eq!(re, Ok(Mask(true, vec!["a".to_string()], true)));
    let re = str_to_mask("-ab-");
    assert_eq!(re, Ok(Mask(true, vec!["ab".to_string()], true)));
    let re = str_to_mask("-a-b-");
    assert_eq!(
        re,
        Ok(Mask(true, vec!["a".to_string(), "b".to_string()], true))
    );
    let re = str_to_mask("--a-b-");
    assert!(re.is_err());
    let re = str_to_mask("a--b-");
    assert!(re.is_err());
    let re = str_to_mask("a-1-b-");
    assert!(re.is_err());
}
#[test]
fn to_end() {
    let re = str_to_mask("-a-");
    assert_eq!(re.clone(), Ok(Mask(true, vec!["a".to_string()], true)));
    let reg = re.clone().unwrap().to_regex_str();
    assert_eq!(reg, "^.+?(a).+$");
    let code = "yvgzgbyfahcahbxrtgsjgqjpuglb";
    let mask = re.clone().unwrap();
    let result = mask.apply_to_code(code);
    assert_eq!(result.clone().unwrap(), MaskResult::Matched(vec![(8, 9)]));
    let piece = Pieces::from_code_and_result(code, result.unwrap());
    assert_eq!(
        piece,
        Pieces(vec![
            ("yvgzgbyf".to_string(), false),
            ("a".to_string(), true),
            ("hcahbxrtgsjgqjpuglb".to_string(), false)
        ])
    )
}

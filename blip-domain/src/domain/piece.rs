use std::iter::once;

use super::mask::MaskResult;
use serde::{Deserialize, Serialize};
use MaskResult::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Pieces(pub Vec<(String, bool)>);

fn merge_same(v: Vec<(String, bool)>) -> Vec<(String, bool)> {
    v.into_iter().fold(Vec::new(), |a, b| {
        if a.is_empty() {
            vec![b]
        } else {
            let last = a.last().unwrap();
            if last.1 == b.1 {
                let new_last = (format!("{}{}", last.0, b.0), last.1);
                a[0..a.len() - 1]
                    .to_vec()
                    .into_iter()
                    .chain(once(new_last))
                    .collect()
            } else {
                a.into_iter().chain(once(b)).collect()
            }
        }
    })
}

impl Pieces {
    pub fn from_code_and_result(code: &str, result: MaskResult) -> Self {
        fn include_in_result(i: usize, v: &Vec<(usize, usize)>) -> bool {
            v.into_iter().any(|v| i >= v.0 && i < v.1)
        }
        match result {
            NoMatch => Pieces(vec![(code.to_string(), false)]),
            Matched(v) => Pieces(merge_same(
                code.chars()
                    .enumerate()
                    .map(|(i, c)| (c.to_string(), include_in_result(i, &v)))
                    .collect::<Vec<(String, bool)>>(),
            )),
        }
    }
    pub fn from_code_and_head_len(code: &str, head_len: usize) -> Self {
        match head_len {
            0 => Pieces(vec![(code.to_string(), false)]),
            _ => Pieces(vec![
                (code[0..head_len].to_string(), true),
                (code[head_len..code.len()].to_string(), false),
            ]),
        }
    }
}
#[cfg(test)]
#[test]
fn from_head() {
    let code = "abcd";

    let p = Pieces::from_code_and_head_len(code, 0);
    assert_eq!(p, Pieces(vec![("abcd".to_string(), false)]));
    let p = Pieces::from_code_and_head_len(code, 1);
    assert_eq!(
        p,
        Pieces(vec![("a".to_string(), true), ("bcd".to_string(), false)])
    );
    let p = Pieces::from_code_and_head_len(code, 2);
    assert_eq!(
        p,
        Pieces(vec![("ab".to_string(), true), ("cd".to_string(), false)])
    );
}
#[test]
fn from_result() {
    let code = "abcd";
    let result = MaskResult::Matched(vec![(0, 1)]);
    let p = Pieces::from_code_and_result(code, result);
    assert_eq!(
        p,
        Pieces(vec![("a".to_string(), true), ("bcd".to_string(), false),])
    );
    let result = MaskResult::Matched(vec![(0, 1), (1, 2)]);
    let p = Pieces::from_code_and_result(code, result);
    assert_eq!(
        p,
        Pieces(vec![("ab".to_string(), true), ("cd".to_string(), false),])
    );
    let result = MaskResult::Matched(vec![(0, 2)]);
    let p = Pieces::from_code_and_result(code, result);
    assert_eq!(
        p,
        Pieces(vec![("ab".to_string(), true), ("cd".to_string(), false),])
    )
}
#[test]
fn merge() {
    let v = vec![
        ("a".to_string(), true),
        ("b".to_string(), true),
        ("c".to_string(), false),
        ("d".to_string(), false),
    ];
    let re = vec![("ab".to_string(), true), ("cd".to_string(), false)];
    assert_eq!(re, merge_same(v));
    let v = vec![
        ("a".to_string(), false),
        ("b".to_string(), true),
        ("c".to_string(), false),
        ("d".to_string(), false),
    ];
    let re = vec![
        ("a".to_string(), false),
        ("b".to_string(), true),
        ("cd".to_string(), false),
    ];
    assert_eq!(re, merge_same(v));
    let v = vec![
        ("a".to_string(), false),
        ("b".to_string(), true),
        ("c".to_string(), false),
        ("d".to_string(), true),
    ];
    let re = vec![
        ("a".to_string(), false),
        ("b".to_string(), true),
        ("c".to_string(), false),
        ("d".to_string(), true),
    ];
    assert_eq!(re, merge_same(v))
}

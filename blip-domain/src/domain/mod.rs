use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::code::hash;

use self::mask::{mask_available, match_on_code};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Item {
    pub author: String,
    pub time: u64,
    pub body: String,
    pub mask: String,
    pub attributes: HashMap<String, String>,
}

impl Item {
    pub fn json(&self) -> String {
        let json = serde_json::to_string(self).unwrap();
        json
    }
    pub fn code(&self) -> String {
        hash(self.json().as_str()).into_iter().collect()
    }
    pub fn masked_code(&self) -> Option<(usize, String)> {
        self.masked(self.code())
    }
    pub fn masked(&self, code: String) -> Option<(usize, String)> {
        // 一般的mask就是level， 1/2/3，
        // 高级mask还没想好，比如可能： c1, c*, *f*
        match self.mask.parse::<usize>() {
            Ok(head) => return Some((0, code[0..head].to_string())),
            _ => match mask_available(&self.mask) {
                Ok(mask_string) => match_on_code(&code, &mask_string)
                    .map(|(start, end)| (start, code[start..end].to_string())),
                _ => None,
            },
        }
    }
}
pub fn god_point() -> Item {
    Item {
        author: "God".to_string(),
        time: 1649249019186u64,
        body: "Hello World".to_string(),
        mask: "1".to_string(),
        attributes: HashMap::new(),
    }
}

mod mask;
mod mask_parse;
#[cfg(test)]
mod test;

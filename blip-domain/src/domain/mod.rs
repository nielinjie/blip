use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::code::hash;

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
    pub fn masked_code(&self) -> Option<String> {
        self.masked(self.code())
    }
    pub fn masked(&self, code: String) -> Option<String> {
        // 一般的mask就是level， 1/2/3，
        // 高级mask还没想好，比如可能： c1, c*, *f*
        match self.mask.parse::<usize>() {
            Ok(head) => return Some(code.chars().take(head).collect()),
            _ => None,
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

#[cfg(test)]
mod test;

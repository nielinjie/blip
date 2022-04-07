use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::code::hash;

#[derive(Serialize, Deserialize,Clone,PartialEq)]
pub struct Item {
    pub author: String,
    pub time: i64,
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
        hash(Item::json(self).as_str(),(self.time % 11) as u8)
    }
}
pub fn god_point() -> Item {
    Item {
        author: "God".to_string(),
        time: 1649249019186i64,
        body: "Hello World".to_string(),
        mask: "1".to_string(),
        attributes: HashMap::new(),
    }
}

#[cfg(test)]
mod test;

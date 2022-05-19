use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::code::hash;

use self::{mask::Mask, piece::Pieces};

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
    pub fn masked_code(&self) -> Result<Pieces, String> {
        self.masked(self.code())
    }
    pub fn masked(&self, code: String) -> Result<Pieces, String> {
        // 一般的mask就是level， 1/2/3，
        // 高级mask还没想好，已经实现的比如：  c-, -f-
        // 可能还有 -c2- ？
        match self.mask.parse::<usize>() {
            Ok(head) => return Ok(Pieces::from_code_and_head_len(&code, head)),
            _ => match Mask::from_str(&self.mask) {
                Ok(mask) => mask
                    .apply_to_code(&code)
                    .map(|result| Pieces::from_code_and_result(&code, result)),
                Err(e) => return Err(e),
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
pub mod piece;
#[cfg(test)]
mod test;

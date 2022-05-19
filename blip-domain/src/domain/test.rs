use chrono::{DateTime, Utc};

use crate::domain::*;

fn god_point_too() -> Item {
    Item {
        author: "God".to_string(),
        time: 1649249019186u64,
        body: "Hello World".to_string(),
        mask: "1".to_string(),
        attributes: HashMap::new(),
    }
}
#[test]
fn json_string() {
    let item = god_point();
    let item2 = god_point();
    let item3 = god_point_too();
    assert_eq!(item.json(), item2.json());
    assert_eq!(item.json(), item3.json())
}
#[test]
fn code_god() {
    let item = god_point();
    let item2 = god_point();
    let item3 = god_point_too();
    assert_eq!(item.code(), item2.code());
    assert_eq!(item.code(), item3.code());
    assert_eq!(item.code(), "chxysmqzlxvjecmfypjawfdsokab"); //len()==28?
    assert_eq!(item.masked_code(),Ok(Pieces(vec![("c".to_string(),true),("hxysmqzlxvjecmfypjawfdsokab".to_string(),false)])));

    let now: DateTime<Utc> = Utc::now();
    let item4 = Item {
        time: now.timestamp_millis() as u64,
        ..item3.clone()
    };
    assert!(!(item4.code() == item3.code()));

    let item4 = Item {
        mask: "2".to_string(),
        ..item3.clone()
    };
    assert!(!(item4.code() == item3.code()));
}

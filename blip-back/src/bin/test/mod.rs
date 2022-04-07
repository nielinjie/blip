#[cfg(test)]
extern crate lazy_static;
use std::{sync::RwLock, io::Error};

use lazy_static::lazy_static;
use reqwest::get;

use crate::Server;
lazy_static! {
    static ref SERVER: RwLock<Server> = RwLock::new(Server::new());
}

#[actix_web::test]
#[ignore]
async fn hello() -> Result<(),Error>{
    server_init().await?;
    let s = get("http://127.0.0.1:8080/").await.unwrap().text().await.unwrap();
    assert_eq!(s, "hello world");
    Ok(())
}

async fn server_init() ->Result<(),Error>{
    SERVER.write().unwrap().start().await
}
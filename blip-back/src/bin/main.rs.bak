use std::io;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    Server::new().start().await;
    println!("server started");
    let s = reqwest::get("http://127.0.0.1:8080/")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(s, "hello world");
    Ok(())
}
struct Server;
impl Server {
    fn new() -> Self {
        Server
    }
    async fn start(&self) -> io::Result<()> {
        HttpServer::new(|| App::new().service(hello))
            .bind("0.0.0.0:8080")?
            .run().await
    }
}


#[cfg(test)]
mod test;
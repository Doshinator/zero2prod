use actix_web::{dev::Server, App, HttpServer};
use std::net::TcpListener;

use crate::routes::{greeting, health_check, hello_world, name, subscribe};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(hello_world)
            .service(greeting)
            .service(name)
            .service(health_check)
            .service(subscribe)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
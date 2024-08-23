use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;

use crate::routes::{greeting, health_check, hello_world, name, subscribe};

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .service(hello_world)
            .service(greeting)
            .service(name)
            .service(health_check)
            .service(subscribe)
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

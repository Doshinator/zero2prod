use actix_web::{dev::Server, get, post, web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World.")
}

#[get("/greeting")]
async fn greeting() -> impl Responder {
    HttpResponse::Ok().body("Greeting!")
}

// to test post enpoint, in terminal
// curl -X POST 127.0.0.1:8080/name -H "Content-Type: plain/text" -d "request body here"
#[post("/name")]
async fn name(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[post("/subscriptions")]
async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

#[allow(dead_code)]
fn index(form: web::Form<FormData>) -> String {
    format!("Welcome {}!, Email {}", form.email, form.name)
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(greeting)
            .service(name)
            .service(health_check)
            .service(subscribe)
    })
    .listen(listener)?
    .run();
    Ok(server)
}

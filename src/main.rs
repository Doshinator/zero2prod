use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(greeting)
            .service(name)
            .service(health_check)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

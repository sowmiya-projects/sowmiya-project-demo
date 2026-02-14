use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <html>
                <head>
                    <title>Rust Web App</title>
                </head>
                <body>
                    <h1>Hey Sowmiya, Your Rust Application is Running</h1>
                    <p>Status: Healthy</p>
                </body>
            </html>
            "#,
        )
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(health)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

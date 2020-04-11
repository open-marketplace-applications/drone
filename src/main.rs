use actix_files as fs;
use actix_web::{middleware, App, HttpServer};
use std::{env};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let port = env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(
                // static files
                fs::Files::new("/", "./frontend/dist/").index_file("index.html"),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
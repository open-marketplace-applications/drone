#[macro_use]
extern crate actix_web;
use actix_cors::Cors;

use std::{env};

use actix_files as fs;
use actix_session::{Session};
use actix_web::{
    http, App, HttpRequest, HttpResponse, HttpServer,
    Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Drone {
    id: u8,
    name: String,
    version: String,
    location: Vec<String>,

}

/// simple index handler
#[get("/drone")]
async fn drone(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    // session
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
    }

    // set counter to session
    session.set("counter", counter)?;

    let data = r#"
    {
        "id": 42,
        "name": "Simulated Drone 42",
        "version": "0.1.4",
        "location": [
                "52.529797",
                "13.413094"
            ]
    }"#;

    let d: Drone = serde_json::from_str(data)?;
    Ok(HttpResponse::Ok().json(d)) // <- send response
}

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
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish())
            .service(drone)
            .service(
                // static files
                fs::Files::new("/", "./frontend/dist/").index_file("index.html"),
            )

    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
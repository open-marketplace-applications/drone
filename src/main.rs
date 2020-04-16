#[macro_use]
extern crate actix_web;
use ::drone::simulator::Drone;
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use std::env;
use std::sync::Mutex;

#[get("/drone")]
async fn drone(req: HttpRequest, drone: web::Data<Mutex<Drone>>) -> Result<HttpResponse> {
    // println!("{:?}", req);

    let mut drone = drone.lock().unwrap();
    drone.update();

    let response = HttpResponse::Ok().json(drone.get_serialized());
    Ok(response)
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
            .data(Mutex::new(Drone::new_from_target(vec![
                // Berlin Tv Tower
                "52.520642".to_owned(),
                "13.409398".to_owned(),
            ])))
            // enable logger
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
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

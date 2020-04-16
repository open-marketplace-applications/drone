#[macro_use]
extern crate actix_web;
use ::drone::simulator::{Drone, State};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use rand::prelude::*;
use std::env;
use std::sync::Mutex;

#[get("/drone")]
async fn drone(req: HttpRequest, drone: web::Data<Mutex<Drone>>) -> Result<HttpResponse> {
    // println!("{:?}", req);

    let mut drone = drone.lock().unwrap();
    drone.update();

    match drone.state() {
        State::Idle => {
            let new_lat = thread_rng().gen_range(52.49, 52.55).to_string();
            let new_long = thread_rng().gen_range(13.39, 13.45).to_string();
            drone.new_target(new_lat.to_owned(), new_long.to_owned());
            drone.state = State::Operating;
        }
        _ => (),
    }

    let response = HttpResponse::Ok().json(drone.clone());
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
            .data(Mutex::new(Drone::new_with_target(vec![
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

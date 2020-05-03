#[macro_use]
extern crate actix_web;
use ::drone::simulator::{Drone, State};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use rand::prelude::*;
use std::env;
use std::sync::Mutex;

use iota_streams::{
    app_channels::api::tangle::{Author}
};
use iota_lib_rs::prelude::iota_client;
use iota_streams::app::transport::tangle::client::SendTrytesOptions;
use crate::author::announce::start_a_new_channel;
use crate::author::send_message::send_signed_message;
mod author;

#[get("/drone")]
async fn drone(req: HttpRequest, drone: web::Data<Mutex<Drone>>) -> Result<HttpResponse> {
    // println!("{:?}", req);

    let mut drone = drone.lock().unwrap();
    drone.update();

    match drone.state() {
        State::Idle => {
            let new_lat = thread_rng().gen_range(52.49, 52.55).to_string();
            let new_long = thread_rng().gen_range(13.39, 13.45).to_string();
            drone.set_target(new_lat.to_owned(), new_long.to_owned());
            drone.state = State::Operating;
        }
        _ => (),
    }

    let response = HttpResponse::Ok().json(drone.clone());
    Ok(response)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    //  -------- IOTA network settings ---------

    // Connect to an IOTA node
    let mut client = iota_client::Client::new("https://nodes.comnet.thetangle.org:443");

    // Change the default settings to use a lower minimum weight magnitude for the Devnet
    let mut send_opt = SendTrytesOptions::default();
    // default is 14
    send_opt.min_weight_magnitude = 10;
    send_opt.local_pow = false;

    // --------------- Author -------------------

    // Create a new channel
    // REPLACE THE SECRET WITH YOUR OWN
    let mut author = Author::new("NYAUTHORSECRET", 3, true);

    let channel_address = author.channel_address().to_string();
    println!("Channel address: {}", &channel_address);

    // Send the `Announce` message
    match start_a_new_channel(&mut author, &mut client, send_opt) {
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }


     //  -------- actix_web Web Server ---------
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let port = env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse()
        .expect("PORT must be a number");

    println!("Server listening on port {}", port);

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

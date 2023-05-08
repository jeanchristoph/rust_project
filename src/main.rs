mod handler;
mod status_handler;


use axum::{routing::get, Router};
use std::net::SocketAddr;
use dotenv::dotenv;

use status_handler::status_handler as Status_handler;
use handler::handler as Handler;

#[macro_use]
extern crate log;


#[tokio::main]
async fn main() {

    env_logger::init();
    info!("Starting application");

    // ...
    // loads the environment variables from the ".env" file.
    dotenv().ok();
    // get listening port
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    // ensure port is valid type
    let port: u16 = port.parse().expect("Port should be valid range");
    // build our application with a route
    // let app = Router::new().route("/", get(handler));
    let app = Router::new()
        .route("/", get(Handler))
        .route("/status/:status", get(Status_handler));
    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("listening on {addr}");
    //...

    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


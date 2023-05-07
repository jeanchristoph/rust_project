use axum::{response::Html, response::Redirect, response::Response, routing::get, Router};
use axum::response::IntoResponse;
use axum::http::StatusCode;

use std::net::SocketAddr;
use axum::extract::Path;
use dotenv::dotenv;

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
        .route("/", get(handler))
        .route("/status/:status", get(status_handler));
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

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn status_handler(Path(name): Path<String>) -> Response {
    // get status code from pattern matching
    match name.as_str() {
        // OK
        "200" => (StatusCode::OK, Html("Everything is fine".to_string())).into_response(),
        "201" => (StatusCode::CREATED).into_response(),
        "204" => (StatusCode::NO_CONTENT).into_response(),
        // // redirection
        "301" | "308" => Redirect::permanent("http://www.rust-lang.org").into_response(),
        "302" | "307" => Redirect::temporary("/").into_response(),
        // // Bad request
        "400" => (StatusCode::BAD_REQUEST, Html("Bad request".to_string())).into_response(),
        "401" => (StatusCode::UNAUTHORIZED, Html("Unauthorized".to_string())).into_response(),
        "403" => (StatusCode::FORBIDDEN, Html("Forbidden".to_string())).into_response(),
        "404" => (StatusCode::NOT_FOUND, Html("Not found".to_string())).into_response(),
        "405" => (
            StatusCode::METHOD_NOT_ALLOWED,
            Html("Method not allowed".to_string()),
        )
            .into_response(),

        "422" => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Html("Unprocessable entity".to_string()),
        )
            .into_response(),
        // Internal error
        "502" => (StatusCode::BAD_GATEWAY, Html("Bad gateway".to_string())).into_response(),
        // all others
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response(),
    }
}

use axum::{response::Html, response::Redirect, response::Response, };
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::extract::Path;

pub async fn status_handler(Path(name): Path<String>) -> Response {
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

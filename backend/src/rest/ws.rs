
use actix_web::{get, post,middleware, App, HttpServer, Responder, HttpResponse, };
use crate::constants::APPLICATION_JSON;

#[get("/ws")]
pub async fn start_ws() -> HttpResponse {
    let code = "Test";
      HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(code)
}
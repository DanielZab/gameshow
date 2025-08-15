use actix_web::{get, post, middleware, App, HttpServer, Responder};
use std::{env, io};

use crate::rest::ws;

pub async fn start_rest_listener() -> io::Result<()> {
    println!("Started rest backend on 0.0.0.0:9090");
    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default())
        .service(ws::start_ws)
    }).bind("0.0.0.0:9090")?.run().await
}
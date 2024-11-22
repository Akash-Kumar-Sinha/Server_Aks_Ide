use actix_cors::Cors;
use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger::Env;
use std::env;

use crate::route::auth_route;

mod route;
mod handlers;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello World! from Rust Actix web. We are the Watchers on the Wall.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let port: u16 = env::var("PORT")
        .expect("Please set the PORT environment variable")
        .parse()
        .expect("PORT must be a number");
    let origin =
        env::var("CLIENT_ORIGIN").expect("Please set the CLIENT_ORIGIN environment variable");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allowed_origin(&origin).max_age(3600))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(root)
            .configure(auth_route::auth_route)
    })
    .bind(("127.0.0.1", port))
    .expect(&format!("Can not bind to port {}", port))
    .run()
    .await
}

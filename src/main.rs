use actix_cors::Cors;
use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder, web::Data};
use dotenv::dotenv;
use env_logger::Env;
use std::env;

mod route;
mod handlers;
mod database;

use crate::route::auth_route;

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
    
    let db_url = env::var("DATABASE_URL").expect("Please set the DATABASE_URL environment variable");
    let pool = database::get_pool(&db_url).await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(database::AppState { db: pool.clone() }))
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

use actix::SyncArbiter;
use actix_cors::Cors;
use actix_web::{get, middleware::Logger, web::Data, App, HttpResponse, HttpServer, Responder};

use dotenv::dotenv;
use env_logger::Env;

mod database;
mod handlers;
mod route;
mod db_models;
mod schema;

use crate::route::auth_route;
use database::{get_pool, AppState, DbActor};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello World! from Rust Actix web. We are the Watchers on the Wall.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let port: u16 = std::env::var("PORT")
        .expect("Please set the PORT environment variable")
        .parse()
        .expect("PORT must be a number");
    let origin =
        std::env::var("CLIENT_ORIGIN").expect("Please set the CLIENT_ORIGIN environment variable");

    let db_url =
        std::env::var("DATABASE_URL").expect("Please set the DATABASE_URL environment variable");
    let pool = get_pool(&db_url).await;

    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
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

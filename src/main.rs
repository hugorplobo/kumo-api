use actix_web::{HttpServer, App, web};
use database::Database;
use dotenvy::dotenv;

mod routes;
mod types;
mod auth;
mod database;

use routes::{validate::validate, add::add};
use types::AppState;

fn main() {
    dotenv().ok();
    env_logger::init();

    actix_web::rt::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
    })
    .block_on(async {
        let database = Database::new().await;

        HttpServer::new(move|| {
            App::new()
                .app_data(web::Data::new(AppState { database: database.clone() }))
                .service(validate)
                .service(add)
        })
        .bind(("0.0.0.0", 8080)).unwrap()
        .run()
        .await
    }).unwrap();
}
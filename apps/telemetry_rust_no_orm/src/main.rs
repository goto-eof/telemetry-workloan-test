use async_once::AsyncOnce;
use sqlx::{Pool, Postgres};

use crate::{
    configuration::{
        config_database::{self, check_connection},
        config_loader::Settings,
    },
    route::routes_util::init_routes,
};
mod configuration;
mod controller;
mod dao;
mod route;

use dotenv;

#[macro_use]
extern crate lazy_static;


lazy_static! {
    static ref SETTINGS: Settings = Settings::init_configuration().unwrap();
    static ref DB_POOL: AsyncOnce<Pool<Postgres>> = AsyncOnce::new(async {
        let db = config_database::establish_connection().await;
        db.unwrap()
    });
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    println!("initializing logging...");
    // log4rs::init_file("./log4rs.yml", Default::default()).unwrap();
    let pool = DB_POOL.get().await;
    check_connection().await;
    sqlx::migrate!().run(pool).await.unwrap();
    println!("================================");
    println!("server started on port [{}] :)", SETTINGS.server_port);
    println!("================================");

    warp::serve(init_routes().await)
        .run(([0, 0, 0, 0], SETTINGS.server_port))
        .await;
}

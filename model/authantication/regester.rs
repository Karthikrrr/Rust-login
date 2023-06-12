use sqlx::{Error, Row};
use sqlx::postgres::PgPoolOptions;

pub async fn regpg(){
    dotenv::dotenv().expect("Enable to load the .env file");

    let url = std::env::var("DB_url").expect("Undable to connect Database");

    let pool = PgPoolOptions::new()
    .max_connections(50)
    .connect()
    .await.expect("Enable to connect");
}
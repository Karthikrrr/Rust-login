use sqlx::{Error, Row};
use sqlx::postgres::PgPoolOptions;

pub async fn login_database() -> Result<()>{

    dotenv::dotenv().expect("Error in getting the URL");

    let url = std::env::var("DB_url").expect("Error in assigning the url")

    let mut pool = PgPoolOptions::new()
        .connect(&url)
        .await.expect("Pool Errror in connecting to Database");
    sqlx::migrate!("./migration").run(&pool).await;

    println!("Working!!!!!!!");
    Ok(())
}
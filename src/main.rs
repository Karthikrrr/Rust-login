use std::error::Error;
use sqlx::Connection;
use sqlx::Row;

struct Logindata {
   pub username : String,
   pub password : String
}
async fn create(lg : &Logindata , pool: &sqlx::PgPool) -> Result<() , Box<dyn Error>>{
    let query = "INSERT INTO logindata(username , password) VALUES ($1 , $2)";

    sqlx::query(query)
    .bind(&lg.username)
    .bind(&lg.password)
    .execute(pool)
    .await?;

    Ok(())
}

 #[tokio::main]
async fn main() -> Result<() , sqlx::Error>
{
//using postgres url and connecting
let url = "postgres://testuser:Karthikr@123@localhost:5432/test";
let pool = sqlx::postgres::PgPool::connect(url).await?;

// using migrations files
sqlx::migrate!("./migration").run(&pool).await;

//passing values to the struct
let lg = Logindata{
    username : "karthik".to_string(),
    password : "123123".to_string()
};

create(&lg , &pool).await;

println!("OK {:?}",pool);

Ok(())

}

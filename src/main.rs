use std::error::Error;


use sqlx::Connection;
use sqlx::Row;
use actix_web::{get , post ,web ,App,HttpServer , HttpResponse};
use serde::{Deserialize,Serialize};
use std::sync::Mutex;
// use actix_web::HttpResponse;
use warp::reply::html;
use std::fs;
use serde_json::json;
// use crate::model::authantication::logindatabase::{login_database};
// use crate::controller::auth::login::{getloginpage};
use crate::model::authantication::regester::{regpg};
use actix_web::cookie::Key;
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};

use actix_identity::IdentityMiddleware;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use std::io;



pub(crate) const COOKIE_DURATION: actix_web::cookie::time::Duration =
     actix_web::cookie::time::Duration::minutes(30);

#[tokio::main]
async fn main() -> Result<(),io::Result<()>>{
     let secret_key = Key::generate();


//.wrap(IdentityMiddleware::default())
     #[cfg(feature = "cors_for_local_development")]
         let cookie_secure = false;
     #[cfg(not(feature = "cors_for_local_development"))]
         let cookie_secure = true;
     HttpServer::new(move|| {
          App::new()
              .wrap(IdentityMiddleware::default())
              .wrap(SessionMiddleware::builder(CookieSessionStore::default(),secret_key.clone())
              .cookie_name("adf-obdd-service-auth".to_owned())
              .cookie_secure(cookie_secure)
              .session_lifecycle(PersistentSession::default().session_ttl(COOKIE_DURATION))
              .build()
     )
     // .service(web::resource("/").to(getloginpage))
})
.bind("127.0.0.1:8080").expect("err")
.run()
.await.expect("Error");

Ok(())

}

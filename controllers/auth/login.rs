use crate::model::authentication::regester::regpg;
use actix_web::{
    cookie::{time::Duration, Key},
    error,
    http::StatusCode,
    middleware, App, HttpMessage as _, HttpRequest, HttpServer, Responder,
};

#[derive(Debug, Clone, PartialEq,Deserialize)]
//create struct according to db values

pub struct user {
    username : String,
    password : String,
}

pub async fn getloginpage() -> HttpResponce{
    println!("welcome to login page");
}
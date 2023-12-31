use axum::{Router, routing::post, Json};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookies, Cookie};

use crate::{Error, Result, web};

pub fn routes() -> Router {
    return Router::new().route("/api/login", post(api_login))
}

async fn api_login( cookies: Cookies , payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("-->> {:<12} - /api/login", "HANDLER");


    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    return Ok(body);
}



#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
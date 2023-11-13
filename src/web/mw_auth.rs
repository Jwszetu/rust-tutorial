use axum::{http::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;

use crate::{web::AUTH_TOKEN, Error, Result};


pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN);


    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?; 

    return Ok(next.run(req).await)
}
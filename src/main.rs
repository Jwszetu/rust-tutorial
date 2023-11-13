use crate::model::ModelController;

pub use self::error::{Error, Result};

use axum::{routing::{get, get_service}, Router, response::{Html, IntoResponse, Response}, extract::{Query, Path}, middleware};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use std::net::SocketAddr;
use tower_http::services::ServeDir;


mod error;
mod web;
mod model;


#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_ticket::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all: Router = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region: --- Start Server

    let addr = SocketAddr::from(([127,0,0,1], 3000));
    println!("-->> {:<12} on {}", "LISTENING", addr);
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    // endregion: --- Start Server

    return Ok(())
}

#[derive(Deserialize)]
struct HelloParams {
    name: Option<String>,
}

fn routes_static() -> Router {
    return Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

async fn main_response_mapper(res: Response) -> Response {
    println!("-->> {:<12} - main_response_mappper", "RES_MAPPER");
    println!();
    return  res;
}

// region: --- routes hello

fn routes_hello() -> Router {
    return Router::new()
    .route("/hello", get(handler))
    .route("/hello2/:name", get(handler2));
}

async fn handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("-->> {:<12} - handler", "HANDLER");
    let name  = params.name.as_deref().unwrap_or("World!");
    return Html(format!("Hello, <strong>{name}!!!</strong>"));
}
// endregion: --- handler

// region: --- handler using path
async fn handler2(Path(_name): Path<String>) -> impl IntoResponse { 
    println!("-->> {:<12} - handler2", "HANDLER");
    return Html(format!("Hello, <strong>{_name}!!!</strong>"));
}

// endregion: --- handler using path
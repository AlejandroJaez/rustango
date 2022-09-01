use axum::{
    routing::{get},
    Router,
};
#[path = "./views.rs"] mod views;


pub fn routes()-> Router<>{
    Router::new()
    .route("/", get(views::hello))
}
use axum::{
    routing::{get},
    Router,
};
#[path = "./views.rs"] mod views;

pub fn routes()-> Router<>{
    return Router::new()
    .route("/", get(views::root))                  // `GET /` goes to `root`
}



use axum::{
    Router,
};
#[path = "../apps/pim/urls.rs"] mod pim;


pub fn routes()-> Router<>{
    return Router::new()
    .nest("/", pim::routes())                  // `GET /` goes to `root`
}
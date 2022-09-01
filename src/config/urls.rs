use axum::{
    Router,
};
#[path = "../apps/pim/urls.rs"] mod pim;


pub fn routes()-> Router{
    Router::new()
    .merge(pim::routes())
}
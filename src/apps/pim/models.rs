use diesel::prelude::*;

#[derive(Queryable)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: bool,
}


pub struct Image {
    pub product: Product,
    pub file: String,
}
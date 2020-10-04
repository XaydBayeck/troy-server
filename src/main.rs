#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod routes;
mod article;

use routes::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![index, category, archive, tag])
        .launch();
}

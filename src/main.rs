#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod article;
mod routes;

use routes::*;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![index, in_category, has_tag, articles, article, files],
        )
        .launch();
}

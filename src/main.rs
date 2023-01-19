#[macro_use]
extern crate rocket;
extern crate rocket_dyn_templates;
#[macro_use]
extern crate dotenv_codegen;

use rocket_dyn_templates::Template;
use rocket::fs::{FileServer, relative};

mod routes;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes::get_routes())
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
        .launch()
        .await.expect("TODO: panic message");
}
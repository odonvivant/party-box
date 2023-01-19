#[macro_use]
extern crate rocket;
extern crate rocket_dyn_templates;

use rocket_dyn_templates::Template;

mod routes;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes::get_routes())
        .attach(Template::fairing())
        .launch()
        .await.expect("TODO: panic message");
}
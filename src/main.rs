#[macro_use]
extern crate rocket;

use rocket::routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![index])
        .launch()
        .await.expect("TODO: panic message");
}

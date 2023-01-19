#[macro_use]
extern crate rocket;

mod routes;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes::get_routes())
        .launch()
        .await.expect("TODO: panic message");
}

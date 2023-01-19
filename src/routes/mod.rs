use rocket_dyn_templates::Template;
use rocket::serde::json::json;
use rocket::Route;

#[get("/")]
fn index() -> Template {
    Template::render("base", json!({"greetings":"Hello, World!"}))
}

pub fn get_routes() -> Vec<Route> {
    let mut routes = routes![index];
    routes
}
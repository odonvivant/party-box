use rocket::Route;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

pub fn get_routes() -> Vec<Route> {
    let mut routes = routes![index];
    routes
}
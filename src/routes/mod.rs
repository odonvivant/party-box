use rocket_dyn_templates::Template;
use rocket::serde::json::json;
use rocket::Route;
use url::Url;

#[get("/")]
fn index() -> Template {

    let scope = "user-read-email";
    let url= Url::parse_with_params(dotenv!("SPOTIFY_AUTHORIZE_ENDPOINT"),
                                      &[("client_id", dotenv!("SPOTIFY_CLIENT_ID")),
                                        ("response_type", "token"),
                                        ("scope", scope),
                                        ("redirect_uri", dotenv!("AUTHORIZATION_CALLBACK_ENDPOINT"))])
        .unwrap();
    Template::render("login", json!({"url":url.to_string()}))
}

#[get("/callback")]
fn callback() -> Template {
    Template::render("base", json!({"token":"test"}))
}

pub fn get_routes() -> Vec<Route> {
    let mut routes = routes![index, callback];
    routes
}
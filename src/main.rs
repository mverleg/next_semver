use ::rocket::routes;
use ::rocket::Rocket;
use ::rocket::Build;

#[get("/<part>/<version>")]
fn hello(part: &str, version: &str) -> String {
    "Hello, world!".to_owned()
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![hello])
}

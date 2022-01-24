use ::rocket::routes;
use ::rocket::Rocket;
use ::rocket::Build;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![hello])
}

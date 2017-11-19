extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn start_webserver() {
    rocket::ignite().mount("/", routes![index]).launch();
}

use rocket;
use rocket_contrib::Json;

#[derive(Serialize, Deserialize)]
struct LinkEntry {
    url: String
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/links")]
fn get_links() -> Json<LinkEntry> {
    Json(LinkEntry {
        url: String::from("perdu.com")
    })
}

pub fn start_webserver() {
    rocket::ignite().mount("/", routes![index, get_links]).launch();
}

use rocket;
use rocket::State;
use rocket_contrib::Json;

use std::collections::LinkedList;

#[derive(Serialize, Deserialize, Clone)]
struct LinkEntry {
    url: String
}

#[derive(Serialize, Deserialize, Clone)]
struct LinkEntries {
    links : LinkedList<LinkEntry>
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/links")]
fn get_links<'r>(link_entries : State<'r, LinkEntries>) -> Json<LinkEntries> {
    Json(link_entries.clone())
}

pub fn start_webserver() {

    let mut link_entries = LinkedList::<LinkEntry>::new();
    link_entries.push_back(LinkEntry {
        url: String::from("perdu.com")
    });

    rocket::ignite()
        .mount("/", routes![index, get_links])
        .manage(LinkEntries{links: link_entries})
        .launch();
}

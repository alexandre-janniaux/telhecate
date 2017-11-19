#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate telegram_bot;
extern crate regex;
extern crate rocket;

use telegram_bot::*;
use regex::Regex;
use std::thread;

mod webserver;

fn extract_link(msg: &String) -> Option<String> {
    let match_link = Regex::new(r"https?://[a-zA-Z\./\?=&]+").unwrap();
    match_link
        .find(&msg)
        .map(|m| String::from(m.as_str()))
}

fn handle_message(api: &Api, evt : Message) {
    if let MessageType::Text(msg) = evt.msg {
        let link = extract_link(&msg)
            .map_or(String::from("[None]"), |l| l);
        let r = api.send_message(
            evt.chat.id(),
            format!("Hi, {}! You sent {}, and I found link {}", evt.from.first_name, msg, link),
            None, None, None, None);

        match r {
            Ok(_) => println!("Message sent"),
            Err(_) => println!("Error during message reply")
        }
    }
}

fn main() {
    println!("Hello, world!");

    let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap();

    let thread_api = thread::spawn(move || {
        let mut listener = api.listener(ListeningMethod::LongPoll(None));

        listener.listen(|update| {
            if let Some(m) = update.message {
                handle_message(&api, m);
            }

            Ok(ListeningAction::Continue)
        }).expect("Couldn't bind the server");
    });

    webserver::start_webserver();

    thread_api.join().expect("The webserver has crashed");

    println!("Goodbye !");
}

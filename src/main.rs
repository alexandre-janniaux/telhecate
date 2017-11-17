extern crate telegram_bot;

use telegram_bot::*;

fn handle_message(api: &Api, evt : &Message) {
    if let MessageType::Text(_) = evt.msg {
        let r = api.send_message(
            evt.chat.id(),
            format!("Hi, {}!", evt.from.first_name),
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
    let mut listener = api.listener(ListeningMethod::LongPoll(None));

    listener.listen(|update| {
        if let Some(m) = update.message {
            handle_message(&api, &m);
        }

        Ok(ListeningAction::Continue)
    }).expect("Couldn't bind the server");

    println!("Goodbye !");
}

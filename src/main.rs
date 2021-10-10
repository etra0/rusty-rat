use telegram_bot as tbot;
use std::env;

mod message_handler;
mod actions;

#[tokio::main]
async fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = tbot::Api::new(token);

    let mut stream = api.stream();
    loop {
        tokio::select! {
            msg = crate::message_handler::message_handler(&mut stream) => {
                println!("Something was received!");
            }
        }
    }
}

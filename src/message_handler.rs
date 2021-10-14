use crate::actions::Action;
use telegram_bot as tbot;
use futures::StreamExt;

pub(crate) async fn message_handler(stream: &mut tbot::UpdatesStream)
    -> Result<Option<Action>, Box<dyn std::error::Error>> {
    let update = stream.next().await;
    if update.is_none() {
        return Err("Somehow this is none".into());
    }
    let update = update.unwrap()?;
    if let tbot::UpdateKind::Message(msg) = update.kind {
        if let tbot::MessageKind::Text { ref data, .. } = msg.kind {
            println!("{}: {}", &msg.from.first_name, data);
            return Ok(crate::actions::parse_message(data));
        }
    } else {
        eprintln!("Received another thingy");
    }

    return Ok(None);
}

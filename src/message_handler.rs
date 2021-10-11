use crate::actions::Action;
use telegram_bot as tbot;
use futures::StreamExt;

pub(crate) async fn message_handler(stream: &mut tbot::UpdatesStream)
    -> Result<Action, Box<dyn std::error::Error>> {
    let update = stream.next().await;
    if update.is_none() {
        return Err("Somehow this is none".into());
    }
    let update = update.unwrap()?;
    if let tbot::UpdateKind::Message(msg) = update.kind {
        if let tbot::MessageKind::Text { ref data, .. } = msg.kind {
            println!("{}: {}", &msg.from.first_name, data);
        }
    } else {
        eprintln!("Received another thingy");
    }

    Ok(Action::AddElement {name: "foo".to_string(), url: "bar".to_string() })
}

use rusty_rat::parsers::Page;

#[non_exhaustive]
pub(crate) enum Action {
    AddElement { url: String, name: String }
}

pub(crate) fn parse_message(msg: &str) -> Option<Action> {
    if !msg.is_empty() && msg.chars().nth(0) != Some('/') {
        return None;
    }

    let spl: Vec<&str> = msg.split(" ").collect();
    match &spl[..] {
        ["/add", name, url @ ..] => println!("> add {} with url {:?}", name, url),
        _ => println!("Not a valid command!"),
    };

    return None;
}

fn add_element(name: String, urls: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

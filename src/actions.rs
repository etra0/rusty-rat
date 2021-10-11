use rusty_rat::parsers::Page;

#[non_exhaustive]
pub(crate) enum Action {
    AddElement { url: String, name: String }
}
        

fn add_product(name: String, urls: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

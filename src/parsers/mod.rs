mod falabella;

pub enum Page {
    Unknown,
    Falabella
}

impl Default for Page {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Default)]
pub struct Product {
    name: String,
    price: usize,
    source: Page,
    url: String
}

pub(crate) async fn request_page_txt(url: &str) -> Result<String, reqwest::Error> {
    reqwest::get(url).await?.text().await
}

pub async fn check_prices() -> Vec<Product> {
    return vec![];
}

fn check_product_origin(url: &str) -> Page {
    return Page::Falabella;
}

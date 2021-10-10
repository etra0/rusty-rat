pub enum Page {
    Falabella
}

pub struct Product {
    name: String,
    price: usize,
    source: Page,
    url: String
}

pub async fn check_prices() -> Vec<Product> {
    return vec![];
}

fn check_product_origin(url: &str) -> Page {
    return Page::Falabella;
}

use crate::errors::*;

mod falabella;

#[derive(Debug)]
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

pub(crate) async fn request_page_txt(url: &str) -> Result<String, NetworkError> {
    let response = reqwest::get(url).await;

    if let Err(e) = response {
        if e.is_status() {
            let status_code = e.status().unwrap().as_u16();
            return Err(NetworkError::StatusError(status_code as _));
        } else if e.is_timeout() {
            return Err(NetworkError::Timeout);
        } else {
            return Err(NetworkError::Unknown);
        }
    } 

    Ok(response.unwrap().text().await.unwrap())
}

pub async fn check_prices() -> Vec<Product> {
    return vec![];
}

fn check_product_origin(url: &str) -> Page {
    if url.contains("falabella") { Page::Falabella } 
    else { Page::Unknown }
}

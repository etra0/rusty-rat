use super::Product;
use scraper::{Html, Selector};
use regex::Regex;

lazy_static::lazy_static! {
    static ref SELECTOR: Selector = Selector::parse(".price-0 span").unwrap();
    static ref CLEAN_COMMA: Regex = Regex::new(r"[,.$ ]").unwrap();
}

pub async fn parse(url: String) -> Result<Product, Box<dyn crate::errors::Error>> {
    let mut retries = 5;
    let body = loop {
        match super::request_page_txt(&url).await {
            Ok(bd) => break bd,
            Err(crate::errors::NetworkError::Timeout) => (),
            Err(e) => return Err(e.into())
        }
        retries -= 1;
        if retries <= 0 {
            return Err(crate::errors::InternalError::TimeoutLimit.into())
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    };

    let page = Html::parse_document(&body);
    let relevant_price = page.select(&SELECTOR).next().unwrap().inner_html();
    let relevant_price = CLEAN_COMMA.replace_all(&relevant_price, "");
    println!("{:?}", relevant_price);
    return Ok(Product::default());
}

#[tokio::test]
pub async fn test_falabella() {
    let url = "https://www.falabella.com/falabella-cl/product/5623973/Refrigerador-No-Frost-252-lt-MRFS-2700G333FW/5623973";
    parse(url.to_string()).await;
}

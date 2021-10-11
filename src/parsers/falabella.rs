use super::Product;
use scraper::{Html, Selector};
use regex::Regex;

lazy_static::lazy_static! {
    static ref SELECTOR: Selector = Selector::parse(".price-0 span").unwrap();
    static ref CLEAN_COMMA: Regex = Regex::new(r"[,.$ ]").unwrap();
}

pub async fn parse(url: String) -> Result<Product, reqwest::Error> {
    let body = super::request_page_txt(&url).await?;
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

use scraper::{Html, Selector};
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = match reqwest::get("https://news.ycombinator.com/").await {
        Ok(x) => x,
        Err(_) => {
            println!("{}", "error on /random request");
            process::exit(0x0100);  //exit if error expected during request to /random
        }
    };
    let text = resp.text().await?;

    let document = Html::parse_document(&text);
    let selector = Selector::parse(r#".title .titleline > a"#).unwrap();
    for (index, elem) in document.select(&selector).into_iter().enumerate() {
        for text_node in elem.text() {
            println!("{}. {}", index + 1, text_node);
        }
            
    }
    Ok(())
}


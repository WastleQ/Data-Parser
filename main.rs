use reqwest::Error;
use scraper::{Html, Selector};
use tokio::task;

#[derive(Debug)]
struct ParseData {
    titles: Vec<String>,
    links: Vec<String>,
}

async fn fetch_html(url: String) -> Result<String, Error> {
    let response = reqwest::get(&url).await?;
    response.text().await
}

fn parse_html(html: &str) -> ParseData {
    let document = Html::parse_document(html);
    let title_selector = Selector::parse("h1, h2, h3").unwrap();
    let links_selector = Selector::parse("a").unwrap();

    let titles = document
        .select(&title_selector)
        .map(|element| element.text().collect::<Vec<_>>().join(" "))
        .collect();

    let links = document
        .select(&links_selector)
        .filter_map(|element| element.value().attr("href"))
        .map(String::from)
        .collect();

    ParseData { titles, links }
}

async fn process_urls(urls: Vec<String>) {
    let mut handles = Vec::new();

    for url in urls {
        let handle = task::spawn(async move {
            match fetch_html(url.clone()).await {
                Ok(html) => {
                    let data = parse_html(&html);
                    println!("Parsed data: {:?}", data);
                }
                Err(e) => println!("Failed to fetch URL: {}", e)
            } 
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    //Mini project
    let urls = vec![
        //Enter the site url
        "https://google.com".to_string(),
    ];

    process_urls(urls).await;
}

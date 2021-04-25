use std::env;
use regex::Regex;

// use std::collections::HashMap;
use error_chain::error_chain;


error_chain! {
  foreign_links {
      ReqError(reqwest::Error);
      IoError(std::io::Error);
      UrlParseError(url::ParseError);
      JoinError(tokio::task::JoinError);
  }
}

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let resp = reqwest::get("https://httpbin.org/ip")
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{:#?}", resp);
//     Ok(())
// }
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let link = &args[1];

    get_data(link).await;
}

async fn get_data(link: &str) -> Result<i32> {
    let res = reqwest::get(link).await?.text().await?;

    let re = Regex::new(r"adaptiveFormats.:(?P<data>.+?)},.playerAds.").unwrap();
    let _mat = re.find(&res).unwrap();
    let caps = re.captures(&res).unwrap();
    let text1 = caps.name("data").unwrap().as_str();
    println!("{}", text1);
    
    Ok(1)
}

// "https://www.youtube.com/watch?v=ww3hx_z-Gq8"

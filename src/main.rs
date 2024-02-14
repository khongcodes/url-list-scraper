// use std::{collections::HashMap, hash::Hash};

// use hyper::client;
// use std::io::Read;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let resp = reqwest::blocking::get("https://httpbin.org/ip")?
//     .json::<HashMap<String, String>>();
    
//     println!("{:#?}", resp);
//     Ok(())
// }

// fn main() {
//     let client = Client::new();
//     let mut res = client.get("http://www.bloomberg.com/")
//         .send()
//         .unwrap();
//     let mut body = String::new();
//     res.read_to_string(&mut body).expect("failed to read into string");
//     println!("{}", body);
// }

// use hyper::http::Method;
use reqwest::{ Url, Request, Method, Client };
use url::ParseError;

#[cfg(not(target_arch="wasm32"))]
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    
    let url = Url::parse("https://hyper.rs").expect("URL ParseError");

    eprintln!("Fetching {url:?}...");

    let request: Request = Request::new(Method::GET, url);

    let client: Client = Client::builder()
        .user_agent("kevin")
        .https_only(true)
        .build()?;


    let res = reqwest::RequestBuilder::from_parts(client, request)
        .send()
        .await?;

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;
    
    println!("{:?}", body.contains("<iframe"));
    
    Ok(())
}
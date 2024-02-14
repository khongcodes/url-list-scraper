use std::fs;
use reqwest::{ Url, Request, Method, Client };
use regex::Regex;
use std::error::Error;

#[cfg(not(target_arch="wasm32"))]
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let mut results_list = vec![String::from("HAS_YOUTUBE_IFRAME,URL")];

    let user_agent = if let Some(user_agent) = std::env::args().nth(1) {
        user_agent
    } else {
        println!("No user agent provided: using default");
        String::from("user")
    };

    let client: Client = Client::builder()
        .user_agent(user_agent)
        .https_only(true)
        .build()?;

    // IMPORT CSV
    let url_list = if let Ok(vec) = get_list_from_csv() {
        vec
    } else {
        panic!()
    };

    for url in url_list {
        let mut string_output = String::new();

        let url_formatted = Url::parse(&url).expect("URL parse error from vec");

        let request: Request = Request::new(Method::GET, url_formatted);

        eprintln!("Fetching {url:?}...");

        let res = reqwest::RequestBuilder::from_parts(client.clone(), request)
            .send()
            .await?;

        eprintln!("Response: {:?} {}", res.version(), res.status());
        // eprintln!("Headers: {:#?}\n", res.headers());

        let body = res.text().await?;

        // This regex expression searches for iframe code sourced to https://www.youtube.com
        let re = Regex::new(r#"<iframe.*src="https:\/\/www\.youtube\.com.*\".*>"#).unwrap();

        string_output.push_str(&re.is_match(&body).to_string());
        string_output.push(',');
        string_output.push_str(&url);
        results_list.push(string_output);
    }

    write_to_results(results_list);
    
    Ok(())
}

fn get_list_from_csv() -> Result<Vec<String>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("urls.csv")?;
    let mut results: Vec<String> = vec![];

    for result in rdr.records() {
        let record = result?;
        results.push(String::from(record.get(0).unwrap()));
    }

    // println!("{:?}", results);

    Ok(results)
}

fn write_to_results(results: Vec<String>) -> Result<(), std::io::Result<()>> {
    
    let output_str = &results.join("\n");

    let path = "results.csv";

    fs::write(path, output_str);
    // let mut output = File::create(path)?;

    // write!(output, output_str);

    Ok(())
}
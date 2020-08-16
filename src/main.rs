use reqwest::Client;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use std::collections::HashMap;
use std::env;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CLIENT: Client = Client::builder().cookie_store(true).build().unwrap();
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Insufficient number of arguments.");
    }

    let username = args[1].clone();
    let password = args[2].clone();

    let url = "https://ungdomsboligaarhus.dk/user";
    let mut form_build_id = String::new();

    //Fetch /user page and extract anti forgery token
    let get_user_page_result = fetch(url).await;

    match get_user_page_result {
        RequestResult::Ok(resp) => {
            if let Some(text) = resp.text {
                form_build_id = get_html_element_value_by_name("form_build_id", &text);
            }
        }
        RequestResult::Error(resp) => {
            println!("Error fetching {}, with status code: {}", url, resp.status)
        }
    }

    //Use anti forgery token in body and post login request
    let mut body: HashMap<&str, &str> = HashMap::new();

    body.insert("name", &username);
    body.insert("pass", &password);
    body.insert("form_id", "user_login");
    body.insert("op", "Log ind");
    body.insert("form_build_id", &form_build_id);

    let post_login_result = post("https://ungdomsboligaarhus.dk/user", body).await;

    match post_login_result {
        RequestResult::Ok(_) => {
            println!("Successfully reapplied");
        }
        RequestResult::Error(res) => {
            println!("Error posting login form with status: {}", res.status);
        }
    }
}

async fn fetch(url: &str) -> RequestResult {
    let response = CLIENT.get(url).send().await.unwrap();
    let status = response.status();
    let status_code = status.as_u16();

    match status.is_success() {
        true => {
            let response_text = response.text().await.unwrap();

            let request_response = RequestResponse::new(Some(response_text), status_code);

            RequestResult::Ok(request_response)
        }
        false => {
            let request_response = RequestResponse::new(None, status_code);

            RequestResult::Error(request_response)
        }
    }
}

async fn post(url: &str, body_json: HashMap<&str, &str>) -> RequestResult {
    let response = CLIENT.post(url).form(&body_json).send().await.unwrap();
    let status = response.status();
    let status_code = status.as_u16();

    match status.is_success() {
        true => {
            let response_text = response.text().await.unwrap();

            let request_response = RequestResponse::new(Some(response_text), status_code);

            RequestResult::Ok(request_response)
        }
        false => {
            let request_response = RequestResponse::new(None, status_code);

            RequestResult::Error(request_response)
        }
    }
}

fn get_html_element_value_by_name(name: &str, html: &str) -> String {
    let doc = Document::from(html);
    let res = doc
        .find(Name("input").and(Attr("name", name)))
        .filter_map(|n| n.attr("value"))
        .next()
        .unwrap();

    String::from(res)
}

enum RequestResult {
    Ok(RequestResponse),
    Error(RequestResponse),
}

struct RequestResponse {
    text: Option<String>,
    status: u16,
}

impl RequestResponse {
    fn new(text: Option<String>, status: u16) -> RequestResponse {
        RequestResponse { text, status }
    }
}

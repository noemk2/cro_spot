use serde_derive::Deserialize; 
use serde_json::Value as JsonValue;
use std::env;

use std::alloc::System;

#[global_allocator]
static A: System = System;

#[derive(Deserialize)]
struct Response {
    data: Vec<Model>,
}

#[derive(Deserialize)]
struct Model {
    s: String,
    p: f64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let uri = "https://api.crypto.com/v2/public/get-trades?instrument_name=".to_owned() + &args[1].to_uppercase() + "_USDT";

    let response = reqwest::blocking::get(uri).unwrap();
    let response =response.text().unwrap();
    let response = serde_json::from_str::<JsonValue>(&response).unwrap();
    let response =response["result"].to_string();

    let resp = serde_json::from_str::<Response>(&response).unwrap();

    for model in resp.data {
        if model.s == "BUY" {
            println!("{}", model.p);
            break;
        }
    }

}

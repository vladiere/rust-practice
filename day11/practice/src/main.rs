use std::collections::HashMap;

use log::info;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct LoginBody {
    email_address: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct LoginResponse {
    access_token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let server_url = "http://localhost:4000/api/";

    info!("Instantiating reqwest client");
    let client = reqwest::Client::new();

    info!("logging you user credentials");
    let login_url = format!("{}login", &server_url);

    info!("Endpoint: {}", login_url);

    let login_body = LoginBody {
        email_address: String::from("vlad@email.com"),
        password: String::from("123123"),
    };

    let mut req_body = HashMap::new();
    req_body.insert("email_address", "vlad@email.com");
    req_body.insert("password", "123123");

    let login_response = client.post(&login_url).json(&login_body).send().await?;

    info!("login response: {:#?}", login_response);

    Ok(())
}

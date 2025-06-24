use::reqwest::Error;
use reqwest::{header,header::HeaderMap, Client, Response};
use std::env;

struct Env {
    supabase_url: String,
    supabase_key: String,
}

fn get_env() -> Env {
    let supabase_key = env::var("SUPABASE_ANON_KEY")
        .expect("SUPABASE_ANON_KEY must be set in .env file");
    let supabase_url = env::var("SUPABASE_URL")
        .expect("SUPABASE_URL must be set in .env file");

    Env {
        supabase_key: supabase_key,
        supabase_url: supabase_url,
    }
}

fn create_client(api_key: String) -> Client {  
    let mut headers: HeaderMap = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", api_key).parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("apikey", api_key.parse().unwrap());

    Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to build reqwest Client")
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let env: Env = get_env();
    
    let client: Client = create_client(env.supabase_key);

    let api_url = format!("{}/rest/v1/profiles", env.supabase_url);

    let response: Response = client.get(&api_url).send().await?;

    let body: serde_json::Value = response.json().await?;

    println!("{:?}", body);

    Ok(())
}

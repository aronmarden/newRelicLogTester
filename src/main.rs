use reqwest::Client;
use serde_json::json;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::Runtime;

async fn send_log(api_key: &str, message: &str) -> Result<(), reqwest::Error> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let obfuscated_key = format!("{}********", &api_key[..8]);

    let payload = json!({
        "timestamp": timestamp,
        "message": message,
        "logtype": "rust_logger",
        "description": "This Log has come from the rust_logger testing tool. Please see <insert github link> for more details",
        "apiKeyUsed": obfuscated_key,
    });

    let client = Client::new();

    let response = client
        .post("https://log-api.newrelic.com/log/v1")
        .header("Content-Type", "application/json")
        .header("Api-Key", api_key)
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Log sent successfully!");
    } else {
        println!("Failed to send log: {}", response.status());
    }

    Ok(())
}

fn main() {
    println!("Please enter your New Relic API Key:");
    let mut api_key = String::new();
    io::stdin()
        .read_line(&mut api_key)
        .expect("Failed to read API key from stdin");
    let api_key = api_key.trim();

    let rt = Runtime::new().expect("Failed to create runtime");

    println!("Please enter a message, and it will be sent to the New Relic Log API (type 'exit' to quit):");

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut message = String::new();
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read message from stdin");

        let message = message.trim();
        if message.eq_ignore_ascii_case("exit") {
            break;
        }

        rt.block_on(send_log(api_key, message)).expect("Failed to send log");
    }
}

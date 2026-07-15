use serde_json::Value;
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    // Read JSON-RPC messages from stdin in an infinite loop
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    while let Ok(Some(line)) = reader.next_line().await {
        if let Ok(request) = serde_json::from_str::<Value>(&line) {
            handle_request(request).await;
        }
    }
}

async fn handle_request(request: Value) {
    match request["method"].as_str().unwrap_or_default() {
        "initialize" => {
            // Logic for protocol handshake
            println!("Initialization successful");
        }
        "tools/list" => {
            // Logic to list available tools
            println!("Available tools: [\"tool1\", \"tool2\"]");
        }
        _ => {}
    }
}
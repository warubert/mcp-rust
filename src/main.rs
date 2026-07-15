use std::io::Write;

use serde_json::Value;
use serde_json::json;
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
    let id = request["id"].as_str().unwrap_or_default();

    match request["method"].as_str().unwrap_or_default() {
        "initialize" => {
            // Send the protocol handshake response
            send_response(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {"tools": {}},
                    "serverInfo": {"name": "roma-mcp", "version": "0.1.0"}
                }
            }));
        }
        "tools/list" => {
            // Send the list of available tools for Claude
            send_response(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "tools": [
                        {
                            "name": "analyze_production_log",
                            "description": "Reads local production-crash.log to find errors.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {},
                                "required": []
                            }
                        }
                    ]
                }
            }));
        }
        _ => {}
    }
}

fn send_response(response: Value) {
    // Helper to send the JSON response back to Claude via stdout
    let mut stdout = std::io::stdout();
    let output = serde_json::to_string(&response).unwrap();
    writeln!(stdout, "{}", output).unwrap();
    stdout.flush().unwrap();
}
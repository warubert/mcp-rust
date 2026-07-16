# mcp-rust

rodar docker
docker build -t rust_mcp .
docker run -i --rm -v ./production-crash.log:/app/production-crash.log rust_mcp

config do claude
"mcpServers": {
"roma-demo": {
"command": "docker",
"args": [
"run",
"-i",
"--rm",
"-v",
"C:/Users/me/workspace/mcp-server/production-crash.log:/app/production-crash.log",
"roma-mcp"
]
}
},

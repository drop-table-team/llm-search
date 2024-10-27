# LLM Search Output Plugin
This plugin provides a ChatGPT like interface for the knowledge hub.

## Deployment
Use the following command to run `llm-search`:
`ADDRESS="0.0.0.0:8081" OLLAMA_ADDRESS="http://192.168.0.104:11434" QDRANT_ADDRESS="http://192.168.0.111:6334" QDRANT_COLLECTION="data" cargo run --release`

| Name | Description | Example |
| - | - | - |
| `ADDRESS` | The address that the local webserver is listening on | `0.0.0.0:8080` | 
| `MODULE_NAME` | The name of the module (in this case `llm-search`) | `llm-search` |
| `BACKEND_ADDRESS` | The address of the backend | `http://192.168.0.69:80` | 
| `OLLAMA_ADDRESS` | The address of the ollama server | `http://192.168.0.104:11434` | 
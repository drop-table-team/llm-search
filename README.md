# LLM Search Output Plugin
This plugin provides a ChatGPT like interface for the knowledge hub.

## Deployment
Use the following command to run `llm-search`:
`ADDRESS="0.0.0.0:8081" OLLAMA_ADDRESS="http://192.168.0.104:11434" QDRANT_ADDRESS="http://192.168.0.111:6334" QDRANT_COLLECTION="data" cargo run --release`

| Name | Description | Example |
| - | - | - |
| `ADDRESS` | The address that the local webserver is listening on | `0.0.0.0:8081` | 
| `OLLAMA_ADDRESS` | The address of the ollama server | `http://192.168.0.104:11434` | 
| `QDRANT_ADDRESS` | The address of the qdrant server | `http://192.168.0.111:6334` | 
| `QDRANT_ADDRESS` | The name of the qdrant collection | `data` | 
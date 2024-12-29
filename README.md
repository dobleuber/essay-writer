# Essay Writer

An intelligent essay writing assistant powered by AI agents that help with research, planning, writing, and critiquing essays.

## Features

- **Planning**: Automatically generates a structured plan for your essay
- **Research**: Performs comprehensive research on the given topic
- **Writing**: Creates well-structured essay drafts
- **Critique**: Reviews and provides feedback on the essay
- **Multiple Revisions**: Supports multiple revision cycles for essay improvement

## Prerequisites

- Rust (2021 edition)
- OpenAI API key
- Qdrant for vector storage
- Tavily API key for research capabilities

## Installation

1. Clone the repository
2. Create a `.env` file with your environment variables. You can use `.env.example` as a template:
   ```
   # Copy .env.example to .env and fill in your API keys
   cp .env.example .env
   ```
   Required environment variables:
   ```
   OPENAI_API_KEY=your_openai_api_key
   TAVILY_API_KEY=your_tavily_api_key
   ```
3. Build the project:
   ```bash
   cargo build
   ```

## Usage

Run the program with:

```bash
cargo run
```

The program will guide you through the essay writing process, utilizing multiple AI agents to:
1. Create an essay plan
2. Conduct research on the topic
3. Write initial drafts
4. Provide critiques and suggestions for improvement
5. Generate revised versions based on feedback

## Project Structure

- `src/main.rs`: Entry point and orchestration of the agents
- `src/agent.rs`: Implementation of different AI agents
- `src/state.rs`: State management for the essay writing process
- `src/lib.rs`: Core library functionality
- `src/prompts.rs`: AI agent prompts and templates

## Dependencies

- `async-openai`: OpenAI API client
- `langchain`: LangChain integration for AI capabilities
- `qdrant-client`: Vector database client
- `tavily`: Research and information retrieval
- `tokio`: Async runtime
- Additional utilities for error handling and serialization

## License

This project is open source and available under the MIT license.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

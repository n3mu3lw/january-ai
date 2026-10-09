<h1 align="center">january-ai</h1>

<p align="center">Unofficial Rust SDK for January AI</p>

## Installation

Run the below command within your project directory:

```bash
cargo add january-ai tokio --features tokio/full
```

## Documentation

Full crate documentation, including endpoint methods, request models, and response types, is hosted automatically on [docs.rs/january-ai](https://docs.rs/january-ai).

## Quickstart

> [!NOTE]
> Be sure to replace `"your_api_key"` in the example below with your actual January AI API key.

```rust
use january_ai::{JanuaryAI, SearchFoodsQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let client = JanuaryAI::new("your_api_key")?;

    // Build search query filters
    let query = SearchFoodsQuery {
        query: "apple".to_string(),
        limit: Some(5),
        ..Default::default()
    };

    // Execute search directly on the client
    let response = client.search_foods(&query).await?;
    
    println!("Found items count: {}", response.items.len());
    for item in response.items {
        println!("- {}", item.name);
    }

    Ok(())
}
```

## Local Development & Testing

### Prerequisites

* Rust 1.75+ toolchain

### Setup

1. Clone this repository

```sh
git clone https://github.com/n3mu3lw/january-ai.git
```

2. Navigate to the project directory and build the workspace

```sh
cd january-ai
cargo build
```

### Running Integration Tests

The test suite uses [`wiremock`](https://crates.io/crates/wiremock) to run offline integration tests against HTTP fixtures stored in `tests/fixtures/`.

```bash
# Run all tests
cargo test

# Run a specific test suite
cargo test --test water_logs
cargo test --test glucose
cargo test --test restaurants
```

## Contributing

Contributions are much welcome! Feel free to open issues or create pull requests to contribute to this project.

## License

This project is licensed under the [MIT License](LICENSE).

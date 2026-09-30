# january-ai

Unofficial Rust SDK for January AI

## Installation

Run the below command within your project directory:

```bash
cargo add january-ai tokio --features tokio/full
```

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

## Contributing

Contributions are much welcome! Feel free to open issues or create pull requests to contribute to this project.

## License

This project is licensed under the [MIT License](LICENSE).

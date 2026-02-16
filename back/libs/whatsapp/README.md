# WhatsApp Business API Client

A Rust library for sending WhatsApp messages via the Meta Graph API, focused on template messaging.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
whatsapp = { path = "../libs/whatsapp" }
```

## Usage

### Configuration

```rust
use whatsapp::{Client, Config};

let config = Config::new(
    "your_webhook_token",
    "your_phone_number_id",
    "your_graph_api_token",
    "v21.0",
    "https://graph.facebook.com",
);

let client = Client::new(config);
```

### Sending a Template Message

```rust
use whatsapp::{Client, Language};
use whatsapp::templates::{send, Parameters};
use whatsapp::templates::builder::ComponentParam;

let params = Parameters::new()
    .with_header(vec![ComponentParam::Text("Welcome!".into())])
    .with_body(vec![
        ComponentParam::Text("John".into()),
        ComponentParam::Text("12345".into()),
    ]);

let response = send(
    &client,
    "525540128869",
    "my_template_name",
    &params,
    Language::EsMx,
).await?;
```

### Component Parameters

The library supports three types of parameters:

```rust
use whatsapp::templates::builder::ComponentParam;
use serde_json::json;

// Text parameter
ComponentParam::Text("Hello World".into())

// URL button
ComponentParam::Url("https://example.com/path".into())

// Flow button (for WhatsApp Flows)
ComponentParam::Flow {
    token: "flow_token".into(),
    data: json!({"screen": "MAIN"}),
}
```

### Building Templates

```rust
let params = Parameters::new()
    .with_header(vec![ComponentParam::Text("Header text".into())])
    .with_body(vec![ComponentParam::Text("Body text".into())])
    .with_footer(vec![ComponentParam::Text("Footer text".into())])
    .with_buttons(vec![
        ComponentParam::Url("https://example.com".into()),
        ComponentParam::Text("Quick reply".into()),
    ]);
```

### Available Languages

```rust
use whatsapp::Language;

Language::EsMx  // Spanish (Mexico) - default
Language::EsEs  // Spanish (Spain)
Language::EnUs  // English (US)
Language::EnGb  // English (UK)
Language::PtBr  // Portuguese (Brazil)
```

### Phone Number Utilities

Normalize Mexican phone numbers to the required format:

```rust
use whatsapp::utils::phone_number;

phone_number::clean("+5215540128869"); // Returns "525540128869"
phone_number::clean("5540128869");     // Returns "525540128869"
```

## Error Handling

```rust
use whatsapp::WhatsappError;

match result {
    Ok(response) => println!("Message sent: {:?}", response),
    Err(WhatsappError::ClientError { status, message, details }) => {
        eprintln!("API error {}: {}", status, message);
    }
    Err(WhatsappError::ServerError(e)) => {
        eprintln!("Network error: {}", e);
    }
    Err(WhatsappError::ConfigError(msg)) => {
        eprintln!("Config error: {}", msg);
    }
}
```

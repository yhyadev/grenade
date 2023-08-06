# Grenadely Fast Web Framework

Grenade is a web framework focused on simplicity and speed, It's designed for various use cases and emphasizes the following key features:

## Key Features

- **Simplicity and Ease-of-use**: Grenade prioritizes a straightforward and user-friendly development experience.
- **Speed**: We believe that fast is always a good thing, and Grenade is optimized for performance.
- **Type Safety**: Enjoy the benefits of Rust's strong type system, ensuring robust and reliable code.

## Getting Started

### Add Grenade to Your Project

Start by adding the Grenade library to your project's `Cargo.toml` file:

```toml
[dependencies]
grenade = "0.1"
```

### Create Your First Grenade App

```rust
use grenade::*;

fn hello_world(_: Context) -> String {
    "Hello World!".to_string()
}

fn main() {
    let mut app = App::build();

    app.get("/", hello_world);

    app.listen(8080).unwrap();
}
```

### Run Your Project

Use the following command to run your Grenade app:

```bash
$ cargo run
```

Your app will be up and running at `http://localhost:8080/`.

## Documentation

[Auto-generated Documentation](https://docs.rs/grenade)

## Contributing

We welcome and appreciate contributions from the community. There are no strict rules; just make your changes and start a pull request.

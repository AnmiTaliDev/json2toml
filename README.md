# json2toml

A fast and reliable CLI utility to convert between JSON and TOML formats, built with Rust.

## 🚀 Features

- ✅ Convert JSON to TOML
- ✅ Convert TOML to JSON
- ✅ Pretty printing support
- ✅ File output or stdout
- ✅ Colorized terminal output
- ✅ Comprehensive error handling
- ✅ Fast performance
- ✅ Cross-platform support

## 📦 Installation

### From Source

```bash
git clone https://github.com/anmitalidev/json2toml.git
cd json2toml
cargo build --release
```

The binary will be available at `target/release/json2toml`.

### Using Cargo

```bash
cargo install --git https://github.com/anmitalidev/json2toml.git
```

## 🔧 Usage

The utility automatically detects the conversion direction based on the input file extension:

- `.json` files → Convert to TOML
- `.toml` files → Convert to JSON

### Basic Usage

```bash
# Convert JSON to TOML (output to stdout)
json2toml config.json

# Convert TOML to JSON (output to stdout)  
json2toml Cargo.toml

# Convert with output file
json2toml config.json -o config.toml
json2toml Cargo.toml -o package.json

# Convert with pretty printing
json2toml config.json --output config.toml --pretty
```

### Command Line Options

```
json2toml [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input file path

Options:
  -o, --output <OUTPUT>  Output file path (optional, prints to stdout if not provided)
  -p, --pretty           Pretty print the output
  -h, --help             Print help
  -V, --version          Print version
```

## 📚 Examples

### JSON to TOML

**Input (config.json):**
```json
{
  "name": "my-app",
  "version": "1.0.0",
  "dependencies": {
    "serde": "1.0",
    "tokio": "1.0"
  },
  "features": ["default", "json"]
}
```

**Command:**
```bash
json2toml config.json -o config.toml
```

**Output (config.toml):**
```toml
name = "my-app"
version = "1.0.0"
features = ["default", "json"]

[dependencies]
serde = "1.0"
tokio = "1.0"
```

### TOML to JSON
```
json2toml Cargo.toml -o Cargo.json --pretty
```

## 🛠️ Development

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Linting

```bash
cargo clippy
```

### Formatting

```bash
cargo fmt
```

## 📁 Project Structure

```
src/
├── main.rs    # CLI interface and main entry point
├── to_toml.rs # JSON to TOML conversion logic
└── to_json.rs # TOML to JSON conversion logic
Cargo.toml     # Project configuration
README.md      # This file
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 👤 Author

**AnmiTalIDev**
- Email: anmitali198@gmail.com
- GitHub: [@anmitalidev](https://github.com/anmitalidev)

## 🙏 Acknowledgments

- Built with [clap](https://github.com/clap-rs/clap) for CLI parsing
- Uses [serde](https://github.com/serde-rs/serde) for serialization
- Powered by [Rust](https://www.rust-lang.org/) 🦀

---

**Happy converting! 🎉**
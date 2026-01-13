# B58UUID CLI

[![Crates.io](https://img.shields.io/crates/v/b58uuid-cli.svg)](https://crates.io/crates/b58uuid-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

Command-line tool for converting UUIDs to compact 22-character Base58 format.

**Website:** [b58uuid.io](https://b58uuid.io)

## Features

- ✨ Convert UUID to B58UUID (36 chars → 22 chars)
- 🔄 Convert B58UUID back to UUID
- 🎲 Generate random B58UUIDs
- 📝 Batch processing from files
- 🔍 Validate UUID and B58UUID formats
- 🎨 Colored output
- ⚡ Fast and lightweight (~475KB binary)
- 🚀 Zero dependencies at runtime

## Installation

### cargo (Recommended)

```bash
cargo install b58uuid-cli
```

### Homebrew (macOS/Linux)

```bash
brew tap b58uuid/tap
brew install b58uuid
```

### Scoop (Windows)

```powershell
scoop bucket add b58uuid https://github.com/b58uuid/scoop-bucket
scoop install b58uuid
```

### Shell Script (macOS/Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/b58uuid/b58uuid-cli/main/install.sh | sh
```

### PowerShell (Windows)

```powershell
iwr -useb https://raw.githubusercontent.com/b58uuid/b58uuid-cli/main/install.ps1 | iex
```

### Manual Download

Download pre-compiled binaries from [GitHub Releases](https://github.com/b58uuid/b58uuid-cli/releases/latest).

## Usage

### Encode UUID to B58UUID

```bash
# Encode a single UUID
b58uuid encode 550e8400-e29b-41d4-a716-446655440000
# Output: BWBeN28Vb7cMEx7Ym8AUzs

# Encode from stdin
echo "550e8400-e29b-41d4-a716-446655440000" | b58uuid encode

# Encode from file
b58uuid encode --file uuids.txt
```

### Decode B58UUID to UUID

```bash
# Decode a single B58UUID
b58uuid decode BWBeN28Vb7cMEx7Ym8AUzs
# Output: 550e8400-e29b-41d4-a716-446655440000

# Decode from stdin
echo "BWBeN28Vb7cMEx7Ym8AUzs" | b58uuid decode

# Decode from file
b58uuid decode --file b58uuids.txt
```

### Generate Random B58UUIDs

```bash
# Generate one B58UUID
b58uuid generate

# Generate multiple B58UUIDs
b58uuid generate --count 5

# Generate as standard UUID
b58uuid generate --uuid
```

### Validate Format

```bash
# Validate UUID or B58UUID
b58uuid validate 550e8400-e29b-41d4-a716-446655440000
b58uuid validate BWBeN28Vb7cMEx7Ym8AUzs
```

### Options

```bash
# Disable colored output
b58uuid --no-color encode <uuid>

# Show version
b58uuid --version

# Show help
b58uuid --help
b58uuid encode --help
```

## Examples

### Basic Usage

```bash
# Encode
$ b58uuid encode 550e8400-e29b-41d4-a716-446655440000
BWBeN28Vb7cMEx7Ym8AUzs

# Decode
$ b58uuid decode BWBeN28Vb7cMEx7Ym8AUzs
550e8400-e29b-41d4-a716-446655440000

# Generate
$ b58uuid generate
BWBeN28Vb7cMEx7Ym8AUzs
```

### Batch Processing

```bash
# Create a file with UUIDs
cat > uuids.txt << EOF
550e8400-e29b-41d4-a716-446655440000
6ba7b810-9dad-11d1-80b4-00c04fd430c8
6ba7b811-9dad-11d1-80b4-00c04fd430c8
EOF

# Encode all UUIDs
b58uuid encode --file uuids.txt
```

### Pipeline Usage

```bash
# Generate and encode
uuidgen | b58uuid encode

# Encode multiple UUIDs
cat uuids.txt | b58uuid encode

# Decode multiple B58UUIDs
cat b58uuids.txt | b58uuid decode
```

### Validation

```bash
# Validate UUID
$ b58uuid validate 550e8400-e29b-41d4-a716-446655440000
✓ Valid UUID
  UUID:    550e8400-e29b-41d4-a716-446655440000
  B58UUID: BWBeN28Vb7cMEx7Ym8AUzs

# Validate B58UUID
$ b58uuid validate BWBeN28Vb7cMEx7Ym8AUzs
✓ Valid B58UUID
  B58UUID: BWBeN28Vb7cMEx7Ym8AUzs
  UUID:    550e8400-e29b-41d4-a716-446655440000
```

## Why B58UUID?

### Compact Format

- **Standard UUID**: 36 characters
  ```
  550e8400-e29b-41d4-a716-446655440000
  ```

- **B58UUID**: 22 characters (39% shorter)
  ```
  BWBeN28Vb7cMEx7Ym8AUzs
  ```

### Benefits

- ✅ **Shorter URLs**: Better for REST APIs and web applications
- ✅ **URL-Safe**: No special characters that need encoding
- ✅ **Human-Readable**: No confusing characters (0, O, I, l)
- ✅ **Database Efficient**: Smaller indexes, faster queries
- ✅ **Copy-Paste Friendly**: Easier to select and copy

## Performance

- **Binary Size**: ~475KB (optimized for size)
- **Startup Time**: Instant (<5ms)
- **Memory Usage**: Minimal (<5MB)
- **Processing Speed**: Fast (handles millions of UUIDs efficiently)

## Building from Source

```bash
# Clone the repository
git clone https://github.com/b58uuid/b58uuid-cli.git
cd b58uuid-cli

# Build release binary
cargo build --release

# Install locally
cargo install --path .

# Run tests
cargo test
```

## Cross-Compilation

```bash
# Install cross
cargo install cross

# Build for Linux
cross build --release --target x86_64-unknown-linux-gnu

# Build for macOS
cross build --release --target x86_64-apple-darwin
cross build --release --target aarch64-apple-darwin

# Build for Windows
cross build --release --target x86_64-pc-windows-gnu
```

## Libraries

B58UUID is available in multiple languages:

- **Rust**: [b58uuid](https://crates.io/crates/b58uuid)
- **Go**: [b58uuid-go](https://github.com/b58uuid/b58uuid-go)
- **JavaScript**: [b58uuid](https://www.npmjs.com/package/b58uuid)
- **Java**: [b58uuid](https://search.maven.org/artifact/io.b58uuid/b58uuid)
- **Python**: [b58uuid](https://pypi.org/project/b58uuid/)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Links

- **Website**: [b58uuid.io](https://b58uuid.io)
- **Documentation**: [docs.rs/b58uuid-cli](https://docs.rs/b58uuid-cli)
- **Repository**: [github.com/b58uuid/b58uuid-cli](https://github.com/b58uuid/b58uuid-cli)
- **Issues**: [github.com/b58uuid/b58uuid-cli/issues](https://github.com/b58uuid/b58uuid-cli/issues)
- **Crates.io**: [crates.io/crates/b58uuid-cli](https://crates.io/crates/b58uuid-cli)

## Acknowledgments

- Uses Bitcoin's Base58 alphabet
- Built with [clap](https://github.com/clap-rs/clap) for CLI parsing
- Powered by [b58uuid](https://crates.io/crates/b58uuid) library

---

Made with ❤️ by the B58UUID community

# Compression Tool

A command-line tool that implements two compression algorithms: Run-Length Encoding (RLE) and simplified LZ77. Available in both Rust and JavaScript implementations.

## Features

- Compress and decompress files using RLE or LZ77 algorithms
- Support for both Rust and JavaScript implementations
- Docker container support
- Comprehensive test suite
- Performance benchmarking
- Cross-platform compatibility

## Project Structure

```
compression-tool/
├── js/                 # JavaScript implementation
│   ├── src/           # Source code
│   ├── test/          # Test files
│   ├── Dockerfile     # Docker configuration
│   └── package.json   # Dependencies
├── rust/              # Rust implementation
│   ├── src/           # Source code
│   ├── tests/         # Test files
│   └── Dockerfile     # Docker configuration
├── .github/           # GitHub Actions workflows
├── benchmark.sh       # Performance benchmarking script
└── README.md          # This file
```

## Installation

### JavaScript Version

```bash
# Clone the repository
git clone https://github.com/chojuninengu/compression-tool.git
cd compression-tool/js

# Install dependencies
npm install

# Make the script executable
chmod +x index.js
```

### Rust Version

```bash
# Clone the repository
git clone https://github.com/chojuninengu/compression-tool.git
cd compression-tool/rust

# Build the project
cargo build --release
```

### Docker

```bash
# Pull the images
docker pull ghcr.io/chojuninengu/compression-tool-js
docker pull ghcr.io/chojuninengu/compression-tool-rust
```

## Usage

### JavaScript Version

```bash
# Compress a file using RLE
node index.js compress input.txt output.txt.cmp --rle

# Decompress a file using RLE
node index.js decompress output.txt.cmp output.txt --rle

# Compress a file using LZ77
node index.js compress input.txt output.txt.cmp --lz

# Decompress a file using LZ77
node index.js decompress output.txt.cmp output.txt --lz
```

### Rust Version

```bash
# Compress a file using RLE
./target/release/compressor compress input.txt output.txt.cmp --rle

# Decompress a file using RLE
./target/release/compressor decompress output.txt.cmp output.txt --rle

# Compress a file using LZ77
./target/release/compressor compress input.txt output.txt.cmp --lz

# Decompress a file using LZ77
./target/release/compressor decompress output.txt.cmp output.txt --lz
```

### Docker

```bash
# Compress using JavaScript implementation
docker run -v $(pwd):/data ghcr.io/chojuninengu/compression-tool-js compress /data/input.txt /data/output.txt.cmp --rle

# Decompress using Rust implementation
docker run -v $(pwd):/data ghcr.io/chojuninengu/compression-tool-rust decompress /data/output.txt.cmp /data/output.txt --lz
```

## Testing

### JavaScript Tests

```bash
cd js
npm test
```

### Rust Tests

```bash
cd rust
cargo test
```

## Benchmarking

A benchmarking script is provided to compare the performance of both implementations:

```bash
./benchmark.sh
```

The script will:

1. Generate test files of various sizes
2. Compress and decompress using both implementations
3. Measure and compare:
   - Compression time
   - Decompression time
   - Compression ratio
   - Memory usage

## Performance Comparison

The Rust implementation typically offers:

- Faster compression and decompression
- Lower memory usage
- Better handling of large files

The JavaScript implementation provides:

- Easier deployment
- Cross-platform compatibility
- Simpler integration with web applications

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

## License

This project is open source and available under the MIT License.

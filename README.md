# Compression Tool

A command-line tool that implements two compression algorithms: Run-Length Encoding (RLE) and simplified LZ77. Available in both Rust and JavaScript implementations.

## Features

- Compress and decompress files using RLE or LZ77 algorithms
- Support for both Rust and JavaScript implementations
- Docker container support
- Comprehensive test suite
- Performance benchmarking

## Installation

### JavaScript Version

```bash
# Clone the repository
git clone https://github.com/chojuninengu/compression.git
cd compression/js-compressor

# Install dependencies
npm install

# Make the script executable
chmod +x index.js
```

### Rust Version

```bash
# Clone the repository
git clone https://github.com/chojuninengu/compression.git
cd compression/rust-compressor

# Build the project
cargo build --release
```

### Docker

```bash
# Pull the images
docker pull ghcr.io/chojuninengu/js-compressor
docker pull ghcr.io/chojuninengu/rust-compressor
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
docker run -v $(pwd):/data ghcr.io/chojuninengu/js-compressor compress /data/input.txt /data/output.txt.cmp --rle

# Decompress using Rust implementation
docker run -v $(pwd):/data ghcr.io/chojuninengu/rust-compressor decompress /data/output.txt.cmp /data/output.txt --lz
```

## Testing

### JavaScript Tests

```bash
cd js-compressor
npm test
```

### Rust Tests

```bash
cd rust-compressor
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

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

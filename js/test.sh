#!/bin/bash

# Create test directory
mkdir -p test_files
cd test_files

# Test Case 1: Empty file
echo -n "" > empty.txt
echo "Testing empty file..."
node ../index.js compress empty.txt empty.rle --rle
node ../index.js decompress empty.rle empty.decompressed --rle
node ../index.js compress empty.txt empty.lz --lz
node ../index.js decompress empty.lz empty.decompressed --lz

# Test Case 2: Single character repeated
echo -n "AAAAA" > single.txt
echo "Testing single character repeated..."
node ../index.js compress single.txt single.rle --rle
node ../index.js decompress single.rle single.decompressed --rle
node ../index.js compress single.txt single.lz --lz
node ../index.js decompress single.lz single.decompressed --lz

# Test Case 3: Alternating characters
echo -n "ABABAB" > alternating.txt
echo "Testing alternating characters..."
node ../index.js compress alternating.txt alternating.rle --rle
node ../index.js decompress alternating.rle alternating.decompressed --rle
node ../index.js compress alternating.txt alternating.lz --lz
node ../index.js decompress alternating.lz alternating.decompressed --lz

# Test Case 4: Long sequences
echo -n "AAAAABBBCC" > long.txt
echo "Testing long sequences..."
node ../index.js compress long.txt long.rle --rle
node ../index.js decompress long.rle long.decompressed --rle
node ../index.js compress long.txt long.lz --lz
node ../index.js decompress long.lz long.decompressed --lz

# Test Case 5: Mixed content
echo -n "Hello, World! This is a test." > mixed.txt
echo "Testing mixed content..."
node ../index.js compress mixed.txt mixed.rle --rle
node ../index.js decompress mixed.rle mixed.decompressed --rle
node ../index.js compress mixed.txt mixed.lz --lz
node ../index.js decompress mixed.lz mixed.decompressed --lz

# Test Case 6: Binary data
dd if=/dev/urandom of=binary.bin bs=1K count=1
echo "Testing binary data..."
node ../index.js compress binary.bin binary.rle --rle
node ../index.js decompress binary.rle binary.decompressed --rle
node ../index.js compress binary.bin binary.lz --lz
node ../index.js decompress binary.lz binary.decompressed --lz

# Verify results
echo "Verifying results..."
for file in *.txt; do
    if [ -f "${file%.txt}.decompressed" ]; then
        if diff "$file" "${file%.txt}.decompressed" > /dev/null; then
            echo "✅ $file: Decompression successful"
        else
            echo "❌ $file: Decompression failed"
        fi
    fi
done

# Clean up
cd ..
rm -rf test_files 
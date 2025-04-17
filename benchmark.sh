#!/bin/bash

# Create test files
echo "Creating test files..."
mkdir -p test_files

# Generate files of different sizes and patterns
dd if=/dev/urandom of=test_files/random_1M.bin bs=1M count=1
dd if=/dev/zero of=test_files/zeros_1M.bin bs=1M count=1
yes "ABC" | head -c 1M > test_files/repeating_1M.bin
cat /usr/share/dict/words | head -c 1M > test_files/text_1M.bin

# Function to run benchmark
run_benchmark() {
    local impl=$1
    local algo=$2
    local file=$3
    
    echo -e "\nBenchmarking $impl with $algo on $(basename $file)"
    
    # Compression
    echo "Compressing..."
    time docker run -v $(pwd):/data ghcr.io/$GITHUB_REPOSITORY_OWNER/compression-tool-$impl compress /data/$file /data/${file}.cmp --$algo
    
    # Get compressed size
    compressed_size=$(stat -c%s "${file}.cmp")
    original_size=$(stat -c%s "$file")
    ratio=$(echo "scale=2; $compressed_size / $original_size * 100" | bc)
    
    echo "Compression ratio: ${ratio}%"
    
    # Decompression
    echo "Decompressing..."
    time docker run -v $(pwd):/data ghcr.io/$GITHUB_REPOSITORY_OWNER/compression-tool-$impl decompress /data/${file}.cmp /data/${file}.dec --$algo
    
    # Verify
    if cmp -s "$file" "${file}.dec"; then
        echo "Verification: SUCCESS"
    else
        echo "Verification: FAILED"
    fi
    
    # Cleanup
    rm "${file}.cmp" "${file}.dec"
}

# Run benchmarks
for file in test_files/*.bin; do
    for impl in rust js; do
        for algo in rle lz; do
            run_benchmark $impl $algo $file
        done
    done
done

# Cleanup
rm -rf test_files 
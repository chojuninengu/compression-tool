/**
 * Compresses data using Run-Length Encoding (RLE)
 * @param {Buffer} data - Input data to compress
 * @returns {Buffer} - Compressed data
 */
function compress(data) {
    if (!data || data.length === 0) {
        return Buffer.alloc(0);
    }

    const result = [];
    let currentByte = data[0];
    let count = 1;

    for (let i = 1; i < data.length; i++) {
        if (data[i] === currentByte && count < 255) {
            count++;
        } else {
            result.push(count, currentByte);
            currentByte = data[i];
            count = 1;
        }
    }
    result.push(count, currentByte);

    return Buffer.from(result);
}

/**
 * Decompresses RLE-encoded data
 * @param {Buffer} data - Compressed data
 * @returns {Buffer} - Decompressed data
 */
function decompress(data) {
    if (!data || data.length === 0) {
        return Buffer.alloc(0);
    }

    const result = [];
    for (let i = 0; i < data.length; i += 2) {
        const count = data[i];
        const byte = data[i + 1];
        for (let j = 0; j < count; j++) {
            result.push(byte);
        }
    }

    return Buffer.from(result);
}

module.exports = {
    compress,
    decompress
}; 
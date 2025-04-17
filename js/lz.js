/**
 * Compresses data using simplified LZ77 algorithm
 * @param {Buffer} data - Input data to compress
 * @returns {Buffer} - Compressed data
 */
function compress(data) {
    if (!data || data.length === 0) {
        return Buffer.alloc(0);
    }

    const WINDOW_SIZE = 20;
    const result = [];
    let pos = 0;

    while (pos < data.length) {
        let bestMatch = { offset: 0, length: 0 };
        const searchStart = Math.max(0, pos - WINDOW_SIZE);
        const searchEnd = pos;

        // Find the longest match in the sliding window
        for (let offset = searchStart; offset < searchEnd; offset++) {
            let length = 0;
            while (pos + length < data.length && 
                   data[offset + length] === data[pos + length] && 
                   length < 255) {
                length++;
            }
            if (length > bestMatch.length) {
                bestMatch = { offset: pos - offset, length };
            }
        }

        if (bestMatch.length > 2) {
            // Encode as a match
            result.push(0x01, bestMatch.offset, bestMatch.length);
            pos += bestMatch.length;
        } else {
            // Encode as a literal
            result.push(0x00, data[pos]);
            pos++;
        }
    }

    return Buffer.from(result);
}

/**
 * Decompresses LZ77-encoded data
 * @param {Buffer} data - Compressed data
 * @returns {Buffer} - Decompressed data
 */
function decompress(data) {
    if (!data || data.length === 0) {
        return Buffer.alloc(0);
    }

    const result = [];
    let pos = 0;

    while (pos < data.length) {
        const type = data[pos++];
        
        if (type === 0x00) {
            // Literal
            result.push(data[pos++]);
        } else if (type === 0x01) {
            // Match
            const offset = data[pos++];
            const length = data[pos++];
            const start = result.length - offset;
            
            for (let i = 0; i < length; i++) {
                result.push(result[start + i]);
            }
        }
    }

    return Buffer.from(result);
}

module.exports = {
    compress,
    decompress
}; 
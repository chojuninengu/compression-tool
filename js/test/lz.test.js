const assert = require('assert');
const { compress, decompress } = require('../lz');

describe('LZ77 Compression', () => {
    it('should compress and decompress a simple string with repetitions', () => {
        const input = Buffer.from('ABABABABABAB');
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed.toString(), input.toString());
    });

    it('should handle empty input', () => {
        const input = Buffer.alloc(0);
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed.length, 0);
    });

    it('should handle single character', () => {
        const input = Buffer.from('A');
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed.toString(), input.toString());
    });

    it('should handle non-repeating data', () => {
        const input = Buffer.from('ABCDEFGHIJKLMNOPQRSTUVWXYZ');
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed.toString(), input.toString());
    });

    it('should handle mixed data with short and long matches', () => {
        const input = Buffer.from('ABCABCABCXYZXYZXYZ123123123');
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed.toString(), input.toString());
    });

    it('should handle data longer than window size', () => {
        const input = Buffer.alloc(100, 'A');
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed.toString(), input.toString());
    });
}); 
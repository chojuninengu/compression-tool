#!/usr/bin/env node

const fs = require('fs');
const { program } = require('commander');
const rle = require('./rle');
const lz = require('./lz');

program
    .name('compressor')
    .description('A compression tool implementing RLE and LZ77 algorithms')
    .version('1.0.0');

program
    .command('compress <input> <output>')
    .description('Compress a file')
    .option('--rle', 'Use RLE compression')
    .option('--lz', 'Use LZ77 compression')
    .action((input, output, options) => {
        try {
            const data = fs.readFileSync(input);
            let compressed;
            
            if (options.rle) {
                compressed = rle.compress(data);
            } else if (options.lz) {
                compressed = lz.compress(data);
            } else {
                console.error('Please specify compression algorithm (--rle or --lz)');
                process.exit(1);
            }
            
            fs.writeFileSync(output, compressed);
            console.log(`Compressed ${input} to ${output}`);
        } catch (error) {
            console.error('Error:', error.message);
            process.exit(1);
        }
    });

program
    .command('decompress <input> <output>')
    .description('Decompress a file')
    .option('--rle', 'Use RLE decompression')
    .option('--lz', 'Use LZ77 decompression')
    .action((input, output, options) => {
        try {
            const data = fs.readFileSync(input);
            let decompressed;
            
            if (options.rle) {
                decompressed = rle.decompress(data);
            } else if (options.lz) {
                decompressed = lz.decompress(data);
            } else {
                console.error('Please specify decompression algorithm (--rle or --lz)');
                process.exit(1);
            }
            
            fs.writeFileSync(output, decompressed);
            console.log(`Decompressed ${input} to ${output}`);
        } catch (error) {
            console.error('Error:', error.message);
            process.exit(1);
        }
    });

program.parse(process.argv); 
use std::io::{Read, Write};

const MAX_RUN_LENGTH: u8 = 255;

/// Compresses data using Run-Length Encoding (RLE)
pub fn compress<R: Read, W: Write>(input: &mut R, output: &mut W) -> anyhow::Result<()> {
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)?;

    if buffer.is_empty() {
        return Ok(());
    }

    let mut current_byte = buffer[0];
    let mut count = 1;

    for &byte in buffer.iter().skip(1) {
        if byte == current_byte && count < MAX_RUN_LENGTH {
            count += 1;
        } else {
            output.write_all(&[count, current_byte])?;
            current_byte = byte;
            count = 1;
        }
    }
    output.write_all(&[count, current_byte])?;

    Ok(())
}

/// Decompresses RLE-encoded data
pub fn decompress<R: Read, W: Write>(input: &mut R, output: &mut W) -> anyhow::Result<()> {
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)?;

    if buffer.is_empty() {
        return Ok(());
    }

    for chunk in buffer.chunks(2) {
        if chunk.len() != 2 {
            return Err(anyhow::anyhow!("Invalid RLE data format"));
        }
        let count = chunk[0];
        let byte = chunk[1];
        output.write_all(&vec![byte; count as usize])?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_rle_roundtrip() {
        let input = b"AAABBBCCCCCDDDDE";
        let mut compressed = Vec::new();
        compress(&mut Cursor::new(input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(&mut Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed.as_slice());
    }

    #[test]
    fn test_rle_empty() {
        let input = b"";
        let mut compressed = Vec::new();
        compress(&mut Cursor::new(input), &mut compressed).unwrap();
        assert!(compressed.is_empty());

        let mut decompressed = Vec::new();
        decompress(&mut Cursor::new(compressed), &mut decompressed).unwrap();
        assert!(decompressed.is_empty());
    }

    #[test]
    fn test_rle_single_byte() {
        let input = b"A";
        let mut compressed = Vec::new();
        compress(&mut Cursor::new(input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(&mut Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed.as_slice());
    }

    #[test]
    fn test_rle_max_run() {
        let input = vec![b'A'; 255];
        let mut compressed = Vec::new();
        compress(&mut Cursor::new(&input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(&mut Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed);
    }
}

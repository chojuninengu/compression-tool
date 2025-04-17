use std::io::{Read, Write};

const WINDOW_SIZE: usize = 20;
const MAX_MATCH_LENGTH: u8 = 255;

/// Compresses data using simplified LZ77 algorithm
pub fn compress<R: Read, W: Write>(input: &mut R, output: &mut W) -> anyhow::Result<()> {
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)?;

    if buffer.is_empty() {
        return Ok(());
    }

    let mut pos = 0;
    while pos < buffer.len() {
        let mut best_match = (0, 0);
        let search_start = pos.saturating_sub(WINDOW_SIZE);
        let search_end = pos;

        // Find the longest match in the sliding window
        for offset in search_start..search_end {
            let mut length = 0;
            while pos + length < buffer.len()
                && buffer[offset + length] == buffer[pos + length]
                && length < MAX_MATCH_LENGTH as usize
            {
                length += 1;
            }
            if length > best_match.1 {
                best_match = (pos - offset, length);
            }
        }

        if best_match.1 > 2 {
            // Encode as a match
            output.write_all(&[0x01, best_match.0 as u8, best_match.1 as u8])?;
            pos += best_match.1;
        } else {
            // Encode as a literal
            output.write_all(&[0x00, buffer[pos]])?;
            pos += 1;
        }
    }

    Ok(())
}

/// Decompresses LZ77-encoded data
pub fn decompress<R: Read, W: Write>(input: &mut R, output: &mut W) -> anyhow::Result<()> {
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)?;

    if buffer.is_empty() {
        return Ok(());
    }

    let mut result = Vec::new();
    let mut pos = 0;

    while pos < buffer.len() {
        let token_type = buffer[pos];
        pos += 1;

        match token_type {
            0x00 => {
                // Literal
                if pos >= buffer.len() {
                    return Err(anyhow::anyhow!("Invalid LZ77 data format"));
                }
                result.push(buffer[pos]);
                pos += 1;
            }
            0x01 => {
                // Match
                if pos + 1 >= buffer.len() {
                    return Err(anyhow::anyhow!("Invalid LZ77 data format"));
                }
                let offset = buffer[pos] as usize;
                let length = buffer[pos + 1] as usize;
                pos += 2;

                if offset > result.len() {
                    return Err(anyhow::anyhow!("Invalid offset in LZ77 data"));
                }

                let start = result.len() - offset;
                for i in 0..length {
                    result.push(result[start + i]);
                }
            }
            _ => return Err(anyhow::anyhow!("Invalid token type in LZ77 data")),
        }
    }

    output.write_all(&result)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_lz_roundtrip() {
        let input = b"ABABABABABAB";
        let mut compressed = Vec::new();
        compress(&mut Cursor::new(input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(&mut Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed.as_slice());
    }

    #[test]
    fn test_lz_empty() {
        let input = b"";
        let mut compressed = Vec::new();
        compress(&mut Cursor::new(input), &mut compressed).unwrap();
        assert!(compressed.is_empty());

        let mut decompressed = Vec::new();
        decompress(&mut Cursor::new(compressed), &mut decompressed).unwrap();
        assert!(decompressed.is_empty());
    }

    #[test]
    fn test_lz_single_byte() {
        let input = b"A";
        let mut compressed = Vec::new();
        compress(&mut Cursor::new(input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(&mut Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed.as_slice());
    }

    #[test]
    fn test_lz_non_repeating() {
        let input = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut compressed = Vec::new();
        compress(&mut Cursor::new(input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(&mut Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed.as_slice());
    }

    #[test]
    fn test_lz_mixed_data() {
        let input = b"ABCABCABCXYZXYZXYZ123123123";
        let mut compressed = Vec::new();
        compress(&mut Cursor::new(input), &mut compressed).unwrap();

        let mut decompressed = Vec::new();
        decompress(&mut Cursor::new(compressed), &mut decompressed).unwrap();

        assert_eq!(input, decompressed.as_slice());
    }
}

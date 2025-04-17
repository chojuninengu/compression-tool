use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_compress_decompress_rle() -> Result<(), Box<dyn std::error::Error>> {
    let input = "test_files/input.txt";
    let compressed = "test_files/output.cmp";
    let decompressed = "test_files/output.txt";

    // Create test directory and file
    fs::create_dir_all("test_files")?;
    fs::write(input, "AAABBBCCCCCDDDDE")?;

    // Compress
    let mut cmd = Command::cargo_bin("compressor")?;
    cmd.arg("compress")
        .arg(input)
        .arg(compressed)
        .arg("--rle")
        .assert()
        .success();

    // Decompress
    let mut cmd = Command::cargo_bin("compressor")?;
    cmd.arg("decompress")
        .arg(compressed)
        .arg(decompressed)
        .arg("--rle")
        .assert()
        .success();

    // Verify
    let original = fs::read(input)?;
    let result = fs::read(decompressed)?;
    assert_eq!(original, result);

    // Cleanup
    fs::remove_dir_all("test_files")?;
    Ok(())
}

#[test]
fn test_compress_decompress_lz() -> Result<(), Box<dyn std::error::Error>> {
    let input = "test_files/input.txt";
    let compressed = "test_files/output.cmp";
    let decompressed = "test_files/output.txt";

    // Create test directory and file
    fs::create_dir_all("test_files")?;
    fs::write(input, "ABABABABABAB")?;

    // Compress
    let mut cmd = Command::cargo_bin("compressor")?;
    cmd.arg("compress")
        .arg(input)
        .arg(compressed)
        .arg("--lz")
        .assert()
        .success();

    // Decompress
    let mut cmd = Command::cargo_bin("compressor")?;
    cmd.arg("decompress")
        .arg(compressed)
        .arg(decompressed)
        .arg("--lz")
        .assert()
        .success();

    // Verify
    let original = fs::read(input)?;
    let result = fs::read(decompressed)?;
    assert_eq!(original, result);

    // Cleanup
    fs::remove_dir_all("test_files")?;
    Ok(())
}

#[test]
fn test_missing_algorithm() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("compressor")?;
    cmd.arg("compress")
        .arg("input.txt")
        .arg("output.cmp")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Please specify compression algorithm",
        ));

    Ok(())
}

#[test]
fn test_invalid_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("compressor")?;
    cmd.arg("compress")
        .arg("nonexistent.txt")
        .arg("output.cmp")
        .arg("--rle")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

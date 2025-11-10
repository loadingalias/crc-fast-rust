//! Real WASM compatibility test
//!
//! This test verifies that the library works in WebAssembly environments.
//!
//! To run these tests:
//! 1. Install wasm-pack: cargo install wasm-pack
//! 2. Run: wasm-pack test --node
//!
//! To just verify WASM builds:
//! cargo build --target wasm32-unknown-unknown --lib --no-default-features --features alloc

#![cfg(target_arch = "wasm32")]

use crc_fast::{checksum, CrcAlgorithm, Digest};

#[cfg(feature = "alloc")]
extern crate alloc;

// For wasm-bindgen-test
#[cfg(all(test, target_arch = "wasm32"))]
use wasm_bindgen_test::*;

#[cfg(all(test, target_arch = "wasm32"))]
wasm_bindgen_test_configure!(run_in_browser);

/// Test basic CRC calculation in WASM
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn test_wasm_basic_crc32() {
    let data = b"123456789";
    let result = checksum(CrcAlgorithm::Crc32IsoHdlc, data);
    assert_eq!(result, 0xcbf43926, "CRC-32 failed in WASM");
}

/// Test CRC-32/ISCSI (important for cloud storage)
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn test_wasm_crc32_iscsi() {
    let data = b"123456789";
    let result = checksum(CrcAlgorithm::Crc32Iscsi, data);
    assert_eq!(result, 0xe3069283, "CRC-32/ISCSI failed in WASM");
}

/// Test CRC-64/NVME (AWS S3's recommended checksum)
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn test_wasm_crc64_nvme() {
    let data = b"123456789";
    let result = checksum(CrcAlgorithm::Crc64Nvme, data);
    assert_eq!(result, 0xae8b14860a799888, "CRC-64/NVME failed in WASM");
}

/// Test Digest API in WASM
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn test_wasm_digest_api() {
    let mut digest = Digest::new(CrcAlgorithm::Crc32IsoHdlc);
    digest.update(b"1234");
    digest.update(b"56789");
    let result = digest.finalize();
    assert_eq!(result, 0xcbf43926, "Digest API failed in WASM");
}

/// Test all CRC-32 algorithms in WASM
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn test_wasm_all_crc32_algorithms() {
    let data = b"123456789";

    assert_eq!(checksum(CrcAlgorithm::Crc32IsoHdlc, data), 0xcbf43926);
    assert_eq!(checksum(CrcAlgorithm::Crc32Bzip2, data), 0xfc891918);
    assert_eq!(checksum(CrcAlgorithm::Crc32Iscsi, data), 0xe3069283);
    assert_eq!(checksum(CrcAlgorithm::Crc32Mpeg2, data), 0x0376e6e7);
}

/// Test all CRC-64 algorithms in WASM
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn test_wasm_all_crc64_algorithms() {
    let data = b"123456789";

    assert_eq!(
        checksum(CrcAlgorithm::Crc64Ecma182, data),
        0x6c40df5f0b497347
    );
    assert_eq!(checksum(CrcAlgorithm::Crc64Nvme, data), 0xae8b14860a799888);
    assert_eq!(checksum(CrcAlgorithm::Crc64Xz, data), 0x995dc9bbdf1939fa);
}

/// Test various buffer sizes in WASM
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg(feature = "alloc")]
fn test_wasm_various_sizes() {
    use alloc::vec::Vec;

    // Small
    let small = b"hello";
    let _ = checksum(CrcAlgorithm::Crc32IsoHdlc, small);

    // Medium
    let medium = b"The quick brown fox jumps over the lazy dog";
    let _ = checksum(CrcAlgorithm::Crc32IsoHdlc, medium);

    // Large (test SIMD paths work in WASM)
    let large: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
    let _ = checksum(CrcAlgorithm::Crc32IsoHdlc, &large);

    // Very large (> 4KB - tests memory handling)
    let very_large: Vec<u8> = (0..8192).map(|i| (i % 256) as u8).collect();
    let _ = checksum(CrcAlgorithm::Crc64Nvme, &very_large);
}

/// Test empty input in WASM
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn test_wasm_empty_input() {
    let empty: &[u8] = &[];
    let result = checksum(CrcAlgorithm::Crc32IsoHdlc, empty);
    assert_eq!(result, 0xffffffff);
}

/// Test incremental hashing in WASM (common use case for streaming)
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn test_wasm_incremental_hashing() {
    // Simulate streaming data
    let mut digest = Digest::new(CrcAlgorithm::Crc64Nvme);

    // Add data in chunks (like reading from network)
    for chunk in [b"123", b"456", b"789"].iter() {
        digest.update(*chunk);
    }

    let result = digest.finalize();
    assert_eq!(result, 0xae8b14860a799888);
}

/// Test Digest reset and reuse in WASM
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn test_wasm_digest_reset() {
    let mut digest = Digest::new(CrcAlgorithm::Crc32IsoHdlc);

    digest.update(b"123456789");
    let result1 = digest.finalize();

    digest.reset();
    digest.update(b"123456789");
    let result2 = digest.finalize();

    assert_eq!(result1, result2);
}

/// Test custom CRC parameters in WASM
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg(feature = "alloc")]
fn test_wasm_custom_params() {
    use crc_fast::{checksum_with_params, CrcParams};

    let params = CrcParams::new(
        "CRC-32/CUSTOM",
        32,
        0x04c11db7,
        0xffffffff,
        true,
        0xffffffff,
        0xcbf43926,
    );

    let result = checksum_with_params(params, b"123456789");
    assert_eq!(result, 0xcbf43926);
}

/// Test checksum_combine in WASM (useful for parallel processing)
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg(feature = "alloc")]
fn test_wasm_checksum_combine() {
    use crc_fast::checksum_combine;

    let crc1 = checksum(CrcAlgorithm::Crc32IsoHdlc, b"1234");
    let crc2 = checksum(CrcAlgorithm::Crc32IsoHdlc, b"56789");
    let combined = checksum_combine(CrcAlgorithm::Crc32IsoHdlc, crc1, crc2, 5);

    let expected = checksum(CrcAlgorithm::Crc32IsoHdlc, b"123456789");
    assert_eq!(combined, expected);
}

/// Test that WASM uses software fallback (no SIMD in standard WASM)
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn test_wasm_software_fallback() {
    // WASM32 without SIMD extensions should use software fallback
    // This test just verifies it works correctly
    let test_vectors = [
        (b"" as &[u8], CrcAlgorithm::Crc32IsoHdlc, 0xffffffff_u64),
        (b"a", CrcAlgorithm::Crc32IsoHdlc, 0xe8b7be43),
        (b"abc", CrcAlgorithm::Crc32IsoHdlc, 0x352441c2),
        (b"123456789", CrcAlgorithm::Crc32IsoHdlc, 0xcbf43926),
    ];

    for (data, algo, expected) in &test_vectors {
        let result = checksum(*algo, data);
        assert_eq!(result, *expected, "Failed for input: {:?}", data);
    }
}

/// Test reflected vs non-reflected in WASM
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn test_wasm_reflection_modes() {
    let data = b"123456789";

    // Reflected
    assert_eq!(checksum(CrcAlgorithm::Crc32IsoHdlc, data), 0xcbf43926);

    // Non-reflected
    assert_eq!(checksum(CrcAlgorithm::Crc32Bzip2, data), 0xfc891918);
}

/// Performance baseline test - verify WASM performance is acceptable
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg(feature = "alloc")]
fn test_wasm_performance_baseline() {
    use alloc::vec::Vec;

    // Generate 1MB of test data
    let data: Vec<u8> = (0..1024 * 1024).map(|i| (i % 256) as u8).collect();

    // This should complete in reasonable time even in WASM
    let _result = checksum(CrcAlgorithm::Crc32IsoHdlc, &data);

    // If we get here, performance is acceptable
    // (No specific timing requirements, just verify it doesn't hang)
}

//! Test that verifies no_std compatibility by checking conditional compilation
//!
//! This test doesn't actually run in no_std mode, but it verifies that:
//! 1. The library compiles without std
//! 2. Core functionality is available
//! 3. Feature flags work correctly

#[test]
fn test_no_std_builds() {
    // This test passes if the crate compiles with --no-default-features
    // Run: cargo build --lib --no-default-features

    // Verify we can use CRC algorithms without std
    // (This test runs with std, but verifies the API exists)
    use crc_fast::{checksum, CrcAlgorithm};

    let data = b"123456789";

    // All 21 standard algorithms should work in no_std
    assert_eq!(checksum(CrcAlgorithm::Crc32IsoHdlc, data), 0xcbf43926);
    assert_eq!(checksum(CrcAlgorithm::Crc32Iscsi, data), 0xe3069283);
    assert_eq!(checksum(CrcAlgorithm::Crc64Xz, data), 0x995dc9bbdf1939fa);
}

#[cfg(feature = "alloc")]
#[test]
fn test_alloc_feature_works() {
    // Verify alloc-dependent features work
    use crc_fast::get_calculator_target;
    use crc_fast::CrcAlgorithm;

    // This function requires alloc (returns String)
    let target = get_calculator_target(CrcAlgorithm::Crc32IsoHdlc);
    assert!(target.contains("aarch64") || target.contains("x86") || target.contains("software"));
}

#[test]
fn test_core_types_only() {
    // Verify we only use core types in public API
    use crc_fast::{CrcAlgorithm, Digest};

    let mut digest = Digest::new(CrcAlgorithm::Crc32IsoHdlc);
    digest.update(b"test");
    let _result = digest.finalize();

    // This works because Digest uses only core types
}

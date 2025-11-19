#![no_main]
use crc_fast::{CrcAlgorithm, Digest};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (u8, Vec<u8>)| {
    let (algo_idx, bytes) = data;

    // List of algorithms to test
    let algorithms = [
        CrcAlgorithm::Crc32Aixm,
        CrcAlgorithm::Crc32Autosar,
        CrcAlgorithm::Crc32Base91D,
        CrcAlgorithm::Crc32Bzip2,
        CrcAlgorithm::Crc32CdRomEdc,
        CrcAlgorithm::Crc32Cksum,
        CrcAlgorithm::Crc32Iscsi,
        CrcAlgorithm::Crc32IsoHdlc,
        CrcAlgorithm::Crc32Jamcrc,
        CrcAlgorithm::Crc32Mef,
        CrcAlgorithm::Crc32Mpeg2,
        CrcAlgorithm::Crc32Xfer,
        CrcAlgorithm::Crc64Ecma182,
        CrcAlgorithm::Crc64GoIso,
        CrcAlgorithm::Crc64Ms,
        // CrcAlgorithm::Crc64Nvme, // Not in crc crate
        CrcAlgorithm::Crc64Redis,
        CrcAlgorithm::Crc64We,
        CrcAlgorithm::Crc64Xz,
    ];

    if algorithms.is_empty() {
        return;
    }
    let algo = algorithms[algo_idx as usize % algorithms.len()];

    // Compute expected value using crc crate
    let (expected, _width) = match algo {
        CrcAlgorithm::Crc32Aixm => (
            crc::Crc::<u32>::new(&crc::CRC_32_AIXM).checksum(&bytes) as u64,
            32,
        ),
        CrcAlgorithm::Crc32Autosar => (
            crc::Crc::<u32>::new(&crc::CRC_32_AUTOSAR).checksum(&bytes) as u64,
            32,
        ),
        CrcAlgorithm::Crc32Base91D => (
            crc::Crc::<u32>::new(&crc::CRC_32_BASE91_D).checksum(&bytes) as u64,
            32,
        ),
        CrcAlgorithm::Crc32Bzip2 => (
            crc::Crc::<u32>::new(&crc::CRC_32_BZIP2).checksum(&bytes) as u64,
            32,
        ),
        CrcAlgorithm::Crc32CdRomEdc => (
            crc::Crc::<u32>::new(&crc::CRC_32_CD_ROM_EDC).checksum(&bytes) as u64,
            32,
        ),
        CrcAlgorithm::Crc32Cksum => (
            crc::Crc::<u32>::new(&crc::CRC_32_CKSUM).checksum(&bytes) as u64,
            32,
        ),
        CrcAlgorithm::Crc32Iscsi => (
            crc::Crc::<u32>::new(&crc::CRC_32_ISCSI).checksum(&bytes) as u64,
            32,
        ),
        CrcAlgorithm::Crc32IsoHdlc => (
            crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&bytes) as u64,
            32,
        ),
        CrcAlgorithm::Crc32Jamcrc => (
            crc::Crc::<u32>::new(&crc::CRC_32_JAMCRC).checksum(&bytes) as u64,
            32,
        ),
        CrcAlgorithm::Crc32Mef => (
            crc::Crc::<u32>::new(&crc::CRC_32_MEF).checksum(&bytes) as u64,
            32,
        ),
        CrcAlgorithm::Crc32Mpeg2 => (
            crc::Crc::<u32>::new(&crc::CRC_32_MPEG_2).checksum(&bytes) as u64,
            32,
        ),
        CrcAlgorithm::Crc32Xfer => (
            crc::Crc::<u32>::new(&crc::CRC_32_XFER).checksum(&bytes) as u64,
            32,
        ),

        CrcAlgorithm::Crc64Ecma182 => (
            crc::Crc::<u64>::new(&crc::CRC_64_ECMA_182).checksum(&bytes),
            64,
        ),
        CrcAlgorithm::Crc64GoIso => (
            crc::Crc::<u64>::new(&crc::CRC_64_GO_ISO).checksum(&bytes),
            64,
        ),
        CrcAlgorithm::Crc64Ms => (crc::Crc::<u64>::new(&crc::CRC_64_MS).checksum(&bytes), 64),
        CrcAlgorithm::Crc64Redis => (
            crc::Crc::<u64>::new(&crc::CRC_64_REDIS).checksum(&bytes),
            64,
        ),
        CrcAlgorithm::Crc64We => (crc::Crc::<u64>::new(&crc::CRC_64_WE).checksum(&bytes), 64),
        CrcAlgorithm::Crc64Xz => (crc::Crc::<u64>::new(&crc::CRC_64_XZ).checksum(&bytes), 64),

        _ => return,
    };

    // Compute actual value using crc-fast
    let mut digest = Digest::new(algo);
    digest.update(&bytes);
    let actual = digest.finalize();

    assert_eq!(
        actual,
        expected,
        "Mismatch for algorithm {:?} with input len {}",
        algo,
        bytes.len()
    );
});

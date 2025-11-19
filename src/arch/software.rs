// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module contains a software fallback for unsupported architectures.

use crate::consts::CRC_64_NVME;
use crate::CrcAlgorithm;
use crate::CrcParams;
#[cfg(feature = "alloc")]
use crc::Algorithm;
use crc::Table;

// Caching for custom CRC algorithms to prevent repeated memory leaks and table regeneration
#[cfg(feature = "alloc")]
#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(feature = "alloc")]
#[cfg(feature = "std")]
use std::sync::{Mutex, OnceLock};

#[cfg(feature = "alloc")]
#[cfg(all(not(feature = "std"), feature = "cache"))]
use hashbrown::HashMap;
#[cfg(feature = "alloc")]
#[cfg(all(not(feature = "std"), feature = "cache"))]
use spin::{Mutex, Once};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;

// Cache key types for custom algorithms
#[cfg(feature = "alloc")]
#[cfg(any(feature = "std", feature = "cache"))]
type Crc32Key = (u32, u32, bool, bool, u32, u32);
#[cfg(feature = "alloc")]
#[cfg(any(feature = "std", feature = "cache"))]
type Crc64Key = (u64, u64, bool, bool, u64, u64);

#[cfg(feature = "alloc")]
#[cfg(any(feature = "std", feature = "cache"))]
type CachedCrc32 = Box<crc::Crc<u32, Table<16>>>;

#[cfg(feature = "alloc")]
#[cfg(any(feature = "std", feature = "cache"))]
type CachedCrc64 = Box<crc::Crc<u64, Table<16>>>;

// Global caches for custom algorithms (std version)
#[cfg(feature = "alloc")]
#[cfg(feature = "std")]
static CUSTOM_CRC32_CACHE: OnceLock<Mutex<HashMap<Crc32Key, CachedCrc32>>> = OnceLock::new();
#[cfg(feature = "alloc")]
#[cfg(feature = "std")]
static CUSTOM_CRC64_CACHE: OnceLock<Mutex<HashMap<Crc64Key, CachedCrc64>>> = OnceLock::new();

// Global caches for custom algorithms (no_std + cache version)
#[cfg(feature = "alloc")]
#[cfg(all(not(feature = "std"), feature = "cache"))]
static CUSTOM_CRC32_CACHE: Once<Mutex<HashMap<Crc32Key, CachedCrc32>>> = Once::new();
#[cfg(feature = "alloc")]
#[cfg(all(not(feature = "std"), feature = "cache"))]
static CUSTOM_CRC64_CACHE: Once<Mutex<HashMap<Crc64Key, CachedCrc64>>> = Once::new();

#[allow(unused)]
const RUST_CRC32_AIXM: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_AIXM);

#[allow(unused)]
const RUST_CRC32_AUTOSAR: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_AUTOSAR);

#[allow(unused)]
const RUST_CRC32_BASE91_D: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_BASE91_D);

#[allow(unused)]
const RUST_CRC32_BZIP2: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_BZIP2);

#[allow(unused)]
const RUST_CRC32_CD_ROM_EDC: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_CD_ROM_EDC);

#[allow(unused)]
const RUST_CRC32_CKSUM: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_CKSUM);

#[allow(unused)]
const RUST_CRC32_ISCSI: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_ISCSI);

#[allow(unused)]
const RUST_CRC32_ISO_HDLC: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_ISO_HDLC);

#[allow(unused)]
const RUST_CRC32_JAMCRC: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_JAMCRC);

#[allow(unused)]
const RUST_CRC32_MEF: crc::Crc<u32, Table<16>> = crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_MEF);

#[allow(unused)]
const RUST_CRC32_MPEG_2: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_MPEG_2);

#[allow(unused)]
const RUST_CRC32_XFER: crc::Crc<u32, Table<16>> =
    crc::Crc::<u32, Table<16>>::new(&crc::CRC_32_XFER);

#[allow(unused)]
const RUST_CRC64_ECMA_182: crc::Crc<u64, Table<16>> =
    crc::Crc::<u64, Table<16>>::new(&crc::CRC_64_ECMA_182);

#[allow(unused)]
const RUST_CRC64_GO_ISO: crc::Crc<u64, Table<16>> =
    crc::Crc::<u64, Table<16>>::new(&crc::CRC_64_GO_ISO);

#[allow(unused)]
const RUST_CRC64_MS: crc::Crc<u64, Table<16>> = crc::Crc::<u64, Table<16>>::new(&crc::CRC_64_MS);

#[allow(unused)]
const RUST_CRC64_NVME: crc::Crc<u64, Table<16>> = crc::Crc::<u64, Table<16>>::new(&CRC_64_NVME);

#[allow(unused)]
const RUST_CRC64_REDIS: crc::Crc<u64, Table<16>> =
    crc::Crc::<u64, Table<16>>::new(&crc::CRC_64_REDIS);

#[allow(unused)]
const RUST_CRC64_WE: crc::Crc<u64, Table<16>> = crc::Crc::<u64, Table<16>>::new(&crc::CRC_64_WE);

#[allow(unused)]
const RUST_CRC64_XZ: crc::Crc<u64, Table<16>> = crc::Crc::<u64, Table<16>>::new(&crc::CRC_64_XZ);

#[allow(unused)]
// Dispatch function that handles the generic case
pub(crate) fn update(state: u64, data: &[u8], params: CrcParams) -> u64 {
    match params.width {
        32 => {
            let params_u32 = match params.algorithm {
                CrcAlgorithm::Crc32Aixm => Some(&RUST_CRC32_AIXM),
                CrcAlgorithm::Crc32Autosar => Some(&RUST_CRC32_AUTOSAR),
                CrcAlgorithm::Crc32Base91D => Some(&RUST_CRC32_BASE91_D),
                CrcAlgorithm::Crc32Bzip2 => Some(&RUST_CRC32_BZIP2),
                CrcAlgorithm::Crc32CdRomEdc => Some(&RUST_CRC32_CD_ROM_EDC),
                CrcAlgorithm::Crc32Cksum => Some(&RUST_CRC32_CKSUM),
                CrcAlgorithm::Crc32Iscsi => Some(&RUST_CRC32_ISCSI),
                CrcAlgorithm::Crc32IsoHdlc => Some(&RUST_CRC32_ISO_HDLC),
                CrcAlgorithm::Crc32Jamcrc => Some(&RUST_CRC32_JAMCRC),
                CrcAlgorithm::Crc32Mef => Some(&RUST_CRC32_MEF),
                CrcAlgorithm::Crc32Mpeg2 => Some(&RUST_CRC32_MPEG_2),
                CrcAlgorithm::Crc32Xfer => Some(&RUST_CRC32_XFER),
                CrcAlgorithm::Crc32Custom => None,
                _ => panic!("Invalid algorithm for u32 CRC"),
            };

            if let Some(p) = params_u32 {
                update_u32(state as u32, data, p) as u64
            } else {
                // Custom CRC
                #[cfg(feature = "alloc")]
                {
                    // Use cache if std or cache feature is enabled
                    #[cfg(any(feature = "std", feature = "cache"))]
                    {
                        let key: Crc32Key = (
                            params.poly as u32,
                            params.init as u32,
                            params.refin,
                            params.refout,
                            params.xorout as u32,
                            params.check as u32,
                        );

                        #[cfg(feature = "std")]
                        {
                            let cache =
                                CUSTOM_CRC32_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
                            let mut cache_guard = cache.lock().unwrap();

                            if let Some(crc) = cache_guard.get(&key) {
                                return update_u32(state as u32, data, crc) as u64;
                            }

                            let algorithm = Algorithm {
                                width: params.width,
                                poly: params.poly as u32,
                                init: params.init as u32,
                                refin: params.refin,
                                refout: params.refout,
                                xorout: params.xorout as u32,
                                check: params.check as u32,
                                residue: 0x00000000,
                            };
                            let static_algorithm = Box::leak(Box::new(algorithm));
                            let crc = Box::new(crc::Crc::<u32, Table<16>>::new(static_algorithm));
                            let result = update_u32(state as u32, data, &crc) as u64;
                            cache_guard.insert(key, crc);
                            result
                        }

                        #[cfg(all(not(feature = "std"), feature = "cache"))]
                        {
                            let cache = CUSTOM_CRC32_CACHE.call_once(|| Mutex::new(HashMap::new()));
                            let mut cache_guard = cache.lock();

                            if let Some(crc) = cache_guard.get(&key) {
                                return update_u32(state as u32, data, crc) as u64;
                            }

                            let algorithm = Algorithm {
                                width: params.width,
                                poly: params.poly as u32,
                                init: params.init as u32,
                                refin: params.refin,
                                refout: params.refout,
                                xorout: params.xorout as u32,
                                check: params.check as u32,
                                residue: 0x00000000,
                            };
                            let static_algorithm = Box::leak(Box::new(algorithm));
                            let crc = Box::new(crc::Crc::<u32, Table<16>>::new(static_algorithm));
                            let result = update_u32(state as u32, data, &crc) as u64;
                            cache_guard.insert(key, crc);
                            result
                        }
                    }

                    // Without cache, just leak (no_std without cache feature)
                    #[cfg(not(any(feature = "std", feature = "cache")))]
                    {
                        let algorithm: Algorithm<u32> = Algorithm {
                            width: params.width,
                            poly: params.poly as u32,
                            init: params.init as u32,
                            refin: params.refin,
                            refout: params.refout,
                            xorout: params.xorout as u32,
                            check: params.check as u32,
                            residue: 0x00000000, // unused in this context
                        };

                        // ugly, but the crc crate is difficult to work with...
                        let static_algorithm = Box::leak(Box::new(algorithm));
                        let crc = crc::Crc::<u32, Table<16>>::new(static_algorithm);
                        update_u32(state as u32, data, &crc) as u64
                    }
                }
                #[cfg(not(feature = "alloc"))]
                panic!("Custom CRC parameters require the 'alloc' feature")
            }
        }
        64 => {
            let params_u64 = match params.algorithm {
                CrcAlgorithm::Crc64Ecma182 => Some(&RUST_CRC64_ECMA_182),
                CrcAlgorithm::Crc64GoIso => Some(&RUST_CRC64_GO_ISO),
                CrcAlgorithm::Crc64Ms => Some(&RUST_CRC64_MS),
                CrcAlgorithm::Crc64Nvme => Some(&RUST_CRC64_NVME),
                CrcAlgorithm::Crc64Redis => Some(&RUST_CRC64_REDIS),
                CrcAlgorithm::Crc64We => Some(&RUST_CRC64_WE),
                CrcAlgorithm::Crc64Xz => Some(&RUST_CRC64_XZ),
                CrcAlgorithm::Crc64Custom => None,
                _ => panic!("Invalid algorithm for u64 CRC"),
            };

            if let Some(p) = params_u64 {
                update_u64(state, data, p)
            } else {
                // Custom CRC
                #[cfg(feature = "alloc")]
                {
                    // Use cache if std or cache feature is enabled
                    #[cfg(any(feature = "std", feature = "cache"))]
                    {
                        let key: Crc64Key = (
                            params.poly,
                            params.init,
                            params.refin,
                            params.refout,
                            params.xorout,
                            params.check,
                        );

                        #[cfg(feature = "std")]
                        {
                            let cache =
                                CUSTOM_CRC64_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
                            let mut cache_guard = cache.lock().unwrap();

                            if let Some(crc) = cache_guard.get(&key) {
                                return update_u64(state, data, crc);
                            }

                            let algorithm = Algorithm {
                                width: params.width,
                                poly: params.poly,
                                init: params.init,
                                refin: params.refin,
                                refout: params.refout,
                                xorout: params.xorout,
                                check: params.check,
                                residue: 0x0000000000000000,
                            };
                            let static_algorithm = Box::leak(Box::new(algorithm));
                            let crc = Box::new(crc::Crc::<u64, Table<16>>::new(static_algorithm));
                            let result = update_u64(state, data, &crc);
                            cache_guard.insert(key, crc);
                            result
                        }

                        #[cfg(all(not(feature = "std"), feature = "cache"))]
                        {
                            let cache = CUSTOM_CRC64_CACHE.call_once(|| Mutex::new(HashMap::new()));
                            let mut cache_guard = cache.lock();

                            if let Some(crc) = cache_guard.get(&key) {
                                return update_u64(state, data, crc);
                            }

                            let algorithm = Algorithm {
                                width: params.width,
                                poly: params.poly,
                                init: params.init,
                                refin: params.refin,
                                refout: params.refout,
                                xorout: params.xorout,
                                check: params.check,
                                residue: 0x0000000000000000,
                            };
                            let static_algorithm = Box::leak(Box::new(algorithm));
                            let crc = Box::new(crc::Crc::<u64, Table<16>>::new(static_algorithm));
                            let result = update_u64(state, data, &crc);
                            cache_guard.insert(key, crc);
                            result
                        }
                    }

                    // Without cache, just leak (no_std without cache feature)
                    #[cfg(not(any(feature = "std", feature = "cache")))]
                    {
                        let algorithm: Algorithm<u64> = Algorithm {
                            width: params.width,
                            poly: params.poly,
                            init: params.init,
                            refin: params.refin,
                            refout: params.refout,
                            xorout: params.xorout,
                            check: params.check,
                            residue: 0x0000000000000000, // unused in this context
                        };

                        // ugly, but the crc crate is difficult to work with...
                        let static_algorithm = Box::leak(Box::new(algorithm));
                        let crc = crc::Crc::<u64, Table<16>>::new(static_algorithm);
                        update_u64(state, data, &crc)
                    }
                }
                #[cfg(not(feature = "alloc"))]
                panic!("Custom CRC parameters require the 'alloc' feature")
            }
        }
        _ => panic!("Unsupported CRC width: {}", params.width),
    }
}

// Specific implementation for u32
fn update_u32(state: u32, data: &[u8], params: &crc::Crc<u32, Table<16>>) -> u32 {
    // apply REFIN if necessary
    let initial = if params.algorithm.refin {
        state.reverse_bits()
    } else {
        state
    };

    let mut digest = params.digest_with_initial(initial);
    digest.update(data);

    let checksum = digest.finalize();

    // remove XOR since this will be applied in the library Digest::finalize() step instead
    checksum ^ params.algorithm.xorout
}

// Specific implementation for u64
fn update_u64(state: u64, data: &[u8], params: &crc::Crc<u64, Table<16>>) -> u64 {
    // apply REFIN if necessary
    let initial = if params.algorithm.refin {
        state.reverse_bits()
    } else {
        state
    };

    let mut digest = params.digest_with_initial(initial);
    digest.update(data);

    // remove XOR since this will be applied in the library Digest::finalize() step instead
    digest.finalize() ^ params.algorithm.xorout
}

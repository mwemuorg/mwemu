use crate::api::windows::common::{crypto_state, ntdll as ntdll_common};
use crate::winapi::helper;

// ═══════════════════════════════════════════════════════════════════════
//  crypto_state: hash lifecycle
// ═══════════════════════════════════════════════════════════════════════

const CALG_MD5: u32 = 0x00008003;
const CALG_SHA1: u32 = 0x00008004;
const CALG_SHA_256: u32 = 0x0000800c;
const CALG_RC4: u32 = 0x00006801;
const CALG_AES_128: u32 = 0x0000660e;
const CALG_AES_256: u32 = 0x00006610;

#[test]
fn test_crypto_state_md5_empty() {
    let h = helper::handler_create("test://md5_empty");
    assert!(crypto_state::create_hash(h, CALG_MD5));

    let digest = crypto_state::get_hash_param(h, crypto_state::HP_HASHVAL).unwrap();
    // MD5("") = d41d8cd98f00b204e9800998ecf8427e
    assert_eq!(digest.len(), 16);
    assert_eq!(digest[0], 0xd4);
    assert_eq!(digest[1], 0x1d);
    assert_eq!(digest[15], 0x7e);

    assert!(crypto_state::destroy_hash(h));
}

#[test]
fn test_crypto_state_md5_hello() {
    let h = helper::handler_create("test://md5_hello");
    assert!(crypto_state::create_hash(h, CALG_MD5));
    assert!(crypto_state::hash_data(h, b"Hello"));

    let digest = crypto_state::get_hash_param(h, crypto_state::HP_HASHVAL).unwrap();
    // MD5("Hello") = 8b1a9953c4611296a827abf8c47804d7
    assert_eq!(digest.len(), 16);
    assert_eq!(digest[0], 0x8b);
    assert_eq!(digest[1], 0x1a);
    assert_eq!(digest[15], 0xd7);

    assert!(crypto_state::destroy_hash(h));
}

#[test]
fn test_crypto_state_md5_incremental() {
    let h = helper::handler_create("test://md5_incr");
    assert!(crypto_state::create_hash(h, CALG_MD5));
    assert!(crypto_state::hash_data(h, b"Hel"));
    assert!(crypto_state::hash_data(h, b"lo"));

    let digest = crypto_state::get_hash_param(h, crypto_state::HP_HASHVAL).unwrap();
    // same as MD5("Hello")
    assert_eq!(digest[0], 0x8b);
    assert_eq!(digest[15], 0xd7);

    assert!(crypto_state::destroy_hash(h));
}

#[test]
fn test_crypto_state_sha1_empty() {
    let h = helper::handler_create("test://sha1_empty");
    assert!(crypto_state::create_hash(h, CALG_SHA1));

    let digest = crypto_state::get_hash_param(h, crypto_state::HP_HASHVAL).unwrap();
    // SHA1("") = da39a3ee5e6b4b0d3255bfef95601890afd80709
    assert_eq!(digest.len(), 20);
    assert_eq!(digest[0], 0xda);
    assert_eq!(digest[19], 0x09);

    assert!(crypto_state::destroy_hash(h));
}

#[test]
fn test_crypto_state_sha1_abc() {
    let h = helper::handler_create("test://sha1_abc");
    assert!(crypto_state::create_hash(h, CALG_SHA1));
    assert!(crypto_state::hash_data(h, b"abc"));

    let digest = crypto_state::get_hash_param(h, crypto_state::HP_HASHVAL).unwrap();
    // SHA1("abc") = a9993e364706816aba3e25717850c26c9cd0d89d
    assert_eq!(digest.len(), 20);
    assert_eq!(digest[0], 0xa9);
    assert_eq!(digest[1], 0x99);
    assert_eq!(digest[19], 0x9d);

    assert!(crypto_state::destroy_hash(h));
}

#[test]
fn test_crypto_state_sha256_empty() {
    let h = helper::handler_create("test://sha256_empty");
    assert!(crypto_state::create_hash(h, CALG_SHA_256));

    let digest = crypto_state::get_hash_param(h, crypto_state::HP_HASHVAL).unwrap();
    // SHA256("") = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
    assert_eq!(digest.len(), 32);
    assert_eq!(digest[0], 0xe3);
    assert_eq!(digest[1], 0xb0);
    assert_eq!(digest[31], 0x55);

    assert!(crypto_state::destroy_hash(h));
}

#[test]
fn test_crypto_state_sha256_hello_world() {
    let h = helper::handler_create("test://sha256_hw");
    assert!(crypto_state::create_hash(h, CALG_SHA_256));
    assert!(crypto_state::hash_data(h, b"hello world"));

    let digest = crypto_state::get_hash_param(h, crypto_state::HP_HASHVAL).unwrap();
    // SHA256("hello world") = b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
    assert_eq!(digest.len(), 32);
    assert_eq!(digest[0], 0xb9);
    assert_eq!(digest[1], 0x4d);
    assert_eq!(digest[31], 0xe9);

    assert!(crypto_state::destroy_hash(h));
}

#[test]
fn test_crypto_state_sha256_incremental() {
    let h = helper::handler_create("test://sha256_incr");
    assert!(crypto_state::create_hash(h, CALG_SHA_256));
    assert!(crypto_state::hash_data(h, b"hello "));
    assert!(crypto_state::hash_data(h, b"world"));

    let digest = crypto_state::get_hash_param(h, crypto_state::HP_HASHVAL).unwrap();
    assert_eq!(digest[0], 0xb9);
    assert_eq!(digest[31], 0xe9);

    assert!(crypto_state::destroy_hash(h));
}

#[test]
fn test_crypto_state_hash_param_size() {
    let h = helper::handler_create("test://hp_size");
    assert!(crypto_state::create_hash(h, CALG_SHA_256));

    let size = crypto_state::get_hash_param(h, crypto_state::HP_HASHSIZE).unwrap();
    let size_val = u32::from_le_bytes([size[0], size[1], size[2], size[3]]);
    assert_eq!(size_val, 32);

    assert!(crypto_state::destroy_hash(h));
}

#[test]
fn test_crypto_state_hash_param_algid() {
    let h = helper::handler_create("test://hp_algid");
    assert!(crypto_state::create_hash(h, CALG_MD5));

    let algid = crypto_state::get_hash_param(h, crypto_state::HP_ALGID).unwrap();
    let algid_val = u32::from_le_bytes([algid[0], algid[1], algid[2], algid[3]]);
    assert_eq!(algid_val, CALG_MD5);

    assert!(crypto_state::destroy_hash(h));
}

#[test]
fn test_crypto_state_invalid_algo() {
    let h = helper::handler_create("test://invalid_algo");
    assert!(!crypto_state::create_hash(h, 0xDEAD));
}

#[test]
fn test_crypto_state_hash_data_after_finalize_fails() {
    let h = helper::handler_create("test://finalize_then_data");
    assert!(crypto_state::create_hash(h, CALG_MD5));
    assert!(crypto_state::hash_data(h, b"test"));

    let _ = crypto_state::get_hash_param(h, crypto_state::HP_HASHVAL).unwrap();

    // after finalization, hash_data should fail
    assert!(!crypto_state::hash_data(h, b"more"));

    assert!(crypto_state::destroy_hash(h));
}

#[test]
fn test_crypto_state_destroy_nonexistent() {
    assert!(!crypto_state::destroy_hash(0xDEAD_BEEF));
}

#[test]
fn test_crypto_state_hash_data_nonexistent() {
    assert!(!crypto_state::hash_data(0xDEAD_BEEF, b"data"));
}

#[test]
fn test_crypto_state_get_param_nonexistent() {
    assert!(crypto_state::get_hash_param(0xDEAD_BEEF, crypto_state::HP_HASHVAL).is_none());
}

#[test]
fn test_crypto_state_get_param_invalid_param() {
    let h = helper::handler_create("test://invalid_param");
    assert!(crypto_state::create_hash(h, CALG_MD5));

    assert!(crypto_state::get_hash_param(h, 0xFF).is_none());

    assert!(crypto_state::destroy_hash(h));
}

// ═══════════════════════════════════════════════════════════════════════
//  crypto_state: key lifecycle + encrypt/decrypt
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_crypto_state_create_key_rc4() {
    let h = helper::handler_create("test://key_rc4");
    assert!(crypto_state::create_key(h, CALG_RC4, 128));

    let kp = crypto_state::get_key_param(h, crypto_state::KP_KEYLEN).unwrap();
    let bits = u32::from_le_bytes([kp[0], kp[1], kp[2], kp[3]]);
    assert_eq!(bits, 128);

    assert!(crypto_state::destroy_key(h));
}

#[test]
fn test_crypto_state_create_key_aes128() {
    let h = helper::handler_create("test://key_aes128");
    assert!(crypto_state::create_key(h, CALG_AES_128, 128));

    let kp_algid = crypto_state::get_key_param(h, crypto_state::KP_ALGID).unwrap();
    let algid = u32::from_le_bytes([kp_algid[0], kp_algid[1], kp_algid[2], kp_algid[3]]);
    assert_eq!(algid, CALG_AES_128);

    let kp_block = crypto_state::get_key_param(h, crypto_state::KP_BLOCKLEN).unwrap();
    let block_bits = u32::from_le_bytes([kp_block[0], kp_block[1], kp_block[2], kp_block[3]]);
    assert_eq!(block_bits, 128);

    assert!(crypto_state::destroy_key(h));
}

#[test]
fn test_crypto_state_create_key_aes256() {
    let h = helper::handler_create("test://key_aes256");
    assert!(crypto_state::create_key(h, CALG_AES_256, 256));

    let kp = crypto_state::get_key_param(h, crypto_state::KP_KEYLEN).unwrap();
    let bits = u32::from_le_bytes([kp[0], kp[1], kp[2], kp[3]]);
    assert_eq!(bits, 256);

    assert!(crypto_state::destroy_key(h));
}

#[test]
fn test_crypto_state_create_key_default_size() {
    let h = helper::handler_create("test://key_default");
    assert!(crypto_state::create_key(h, CALG_RC4, 0));

    let kp = crypto_state::get_key_param(h, crypto_state::KP_KEYLEN).unwrap();
    let bits = u32::from_le_bytes([kp[0], kp[1], kp[2], kp[3]]);
    assert_eq!(bits, 128); // default for RC4

    assert!(crypto_state::destroy_key(h));
}

#[test]
fn test_crypto_state_encrypt_decrypt_roundtrip() {
    let h = helper::handler_create("test://key_enc_dec");
    assert!(crypto_state::create_key(h, CALG_RC4, 128));

    let original = b"Hello, CryptoAPI!".to_vec();
    let mut data = original.clone();

    assert!(crypto_state::encrypt_data(h, &mut data));
    assert_ne!(&data, &original, "encrypted should differ from original");

    assert!(crypto_state::decrypt_data(h, &mut data));
    assert_eq!(&data, &original, "decrypted should match original");

    assert!(crypto_state::destroy_key(h));
}

#[test]
fn test_crypto_state_encrypt_empty() {
    let h = helper::handler_create("test://key_enc_empty");
    assert!(crypto_state::create_key(h, CALG_RC4, 128));

    let mut data = vec![];
    assert!(crypto_state::encrypt_data(h, &mut data));
    assert!(data.is_empty());

    assert!(crypto_state::destroy_key(h));
}

#[test]
fn test_crypto_state_encrypt_nonexistent_key() {
    let mut data = b"test".to_vec();
    assert!(!crypto_state::encrypt_data(0xDEADBEEF, &mut data));
    assert_eq!(&data, b"test", "data unchanged on failure");
}

#[test]
fn test_crypto_state_destroy_key_nonexistent() {
    assert!(!crypto_state::destroy_key(0xDEADBEEF));
}

#[test]
fn test_crypto_state_get_key_param_nonexistent() {
    assert!(crypto_state::get_key_param(0xDEADBEEF, crypto_state::KP_KEYLEN).is_none());
}

#[test]
fn test_crypto_state_get_key_param_invalid() {
    let h = helper::handler_create("test://key_invalid_param");
    assert!(crypto_state::create_key(h, CALG_RC4, 128));
    assert!(crypto_state::get_key_param(h, 0xFF).is_none());
    assert!(crypto_state::destroy_key(h));
}

#[test]
fn test_crypto_state_rc4_blocklen_zero() {
    let h = helper::handler_create("test://rc4_block");
    assert!(crypto_state::create_key(h, CALG_RC4, 128));

    let kp = crypto_state::get_key_param(h, crypto_state::KP_BLOCKLEN).unwrap();
    let block = u32::from_le_bytes([kp[0], kp[1], kp[2], kp[3]]);
    assert_eq!(block, 0, "RC4 is a stream cipher, block length should be 0");

    assert!(crypto_state::destroy_key(h));
}

// ═══════════════════════════════════════════════════════════════════════
//  crypto_state: RC4 known-answer test
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_crypto_state_rc4_known_vector() {
    // RC4 with key = [0x01, 0x02, 0x03, 0x04, 0x05] encrypting [0x00; 8]
    // Known output: 0xb2 0x39 0x63 0x05 0xf0 0x3d 0xc0 0x27
    let h = helper::handler_create("test://rc4_known");
    assert!(crypto_state::create_key(h, CALG_RC4, 40));
    crypto_state::set_key_data(h, &[0x01, 0x02, 0x03, 0x04, 0x05]);

    let mut data = vec![0u8; 8];
    assert!(crypto_state::encrypt_data(h, &mut data));
    assert_eq!(data[0], 0xb2, "RC4 known-answer byte 0");
    assert_eq!(data[1], 0x39, "RC4 known-answer byte 1");
    assert_eq!(data[2], 0x63, "RC4 known-answer byte 2");
    assert_eq!(data[3], 0x05, "RC4 known-answer byte 3");
    assert_eq!(data[4], 0xf0, "RC4 known-answer byte 4");
    assert_eq!(data[5], 0x3d, "RC4 known-answer byte 5");
    assert_eq!(data[6], 0xc0, "RC4 known-answer byte 6");
    assert_eq!(data[7], 0x27, "RC4 known-answer byte 7");

    assert!(crypto_state::destroy_key(h));
}

#[test]
fn test_crypto_state_rc4_roundtrip_long() {
    let h = helper::handler_create("test://rc4_long");
    assert!(crypto_state::create_key(h, CALG_RC4, 128));
    crypto_state::set_key_data(h, b"SecretKey1234567");

    let original: Vec<u8> = (0..256).map(|i| i as u8).collect();
    let mut data = original.clone();

    assert!(crypto_state::encrypt_data(h, &mut data));
    assert_ne!(&data, &original);

    // RC4 is symmetric — create a fresh key with same data for decryption
    assert!(crypto_state::destroy_key(h));
    let h2 = helper::handler_create("test://rc4_long_dec");
    assert!(crypto_state::create_key(h2, CALG_RC4, 128));
    crypto_state::set_key_data(h2, b"SecretKey1234567");

    assert!(crypto_state::decrypt_data(h2, &mut data));
    assert_eq!(&data, &original, "RC4 roundtrip must recover original");

    assert!(crypto_state::destroy_key(h2));
}

// ═══════════════════════════════════════════════════════════════════════
//  crypto_state: AES encrypt/decrypt
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_crypto_state_aes128_roundtrip() {
    let h = helper::handler_create("test://aes128_rt");
    assert!(crypto_state::create_key(h, CALG_AES_128, 128));
    crypto_state::set_key_data(
        h,
        &[
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ],
    );

    let original = [
        0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07,
        0x34,
    ];
    let mut data = original.to_vec();

    assert!(crypto_state::encrypt_data(h, &mut data));
    assert_ne!(&data[..], &original[..], "AES encrypted data must differ");
    assert_eq!(data.len(), 16, "AES-128 ECB preserves block size");

    assert!(crypto_state::decrypt_data(h, &mut data));
    assert_eq!(
        &data[..],
        &original[..],
        "AES-128 roundtrip must recover original"
    );

    assert!(crypto_state::destroy_key(h));
}

#[test]
fn test_crypto_state_aes128_known_vector() {
    // NIST FIPS-197 Appendix B test vector
    let h = helper::handler_create("test://aes128_nist");
    assert!(crypto_state::create_key(h, CALG_AES_128, 128));
    crypto_state::set_key_data(
        h,
        &[
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ],
    );

    let mut data = vec![
        0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07,
        0x34,
    ];

    assert!(crypto_state::encrypt_data(h, &mut data));
    // Expected: 3925841d02dc09fbdc118597196a0b32
    assert_eq!(data[0], 0x39, "AES-128 NIST byte 0");
    assert_eq!(data[1], 0x25, "AES-128 NIST byte 1");
    assert_eq!(data[2], 0x84, "AES-128 NIST byte 2");
    assert_eq!(data[15], 0x32, "AES-128 NIST byte 15");

    assert!(crypto_state::destroy_key(h));
}

#[test]
fn test_crypto_state_aes256_roundtrip() {
    let h = helper::handler_create("test://aes256_rt");
    assert!(crypto_state::create_key(h, CALG_AES_256, 256));
    crypto_state::set_key_data(
        h,
        &[
            0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe, 0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d,
            0x77, 0x81, 0x1f, 0x35, 0x2c, 0x07, 0x3b, 0x61, 0x08, 0xd7, 0x2d, 0x98, 0x10, 0xa3,
            0x09, 0x14, 0xdf, 0xf4,
        ],
    );

    let original = [
        0x6b, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96, 0xe9, 0x3d, 0x7e, 0x11, 0x73, 0x93, 0x17,
        0x2a,
    ];
    let mut data = original.to_vec();

    assert!(crypto_state::encrypt_data(h, &mut data));
    assert_ne!(&data[..], &original[..]);

    assert!(crypto_state::decrypt_data(h, &mut data));
    assert_eq!(
        &data[..],
        &original[..],
        "AES-256 roundtrip must recover original"
    );

    assert!(crypto_state::destroy_key(h));
}

#[test]
fn test_crypto_state_aes256_known_vector() {
    // NIST FIPS-197 AES-256 test vector
    let h = helper::handler_create("test://aes256_nist");
    assert!(crypto_state::create_key(h, CALG_AES_256, 256));
    crypto_state::set_key_data(
        h,
        &[
            0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe, 0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d,
            0x77, 0x81, 0x1f, 0x35, 0x2c, 0x07, 0x3b, 0x61, 0x08, 0xd7, 0x2d, 0x98, 0x10, 0xa3,
            0x09, 0x14, 0xdf, 0xf4,
        ],
    );

    let mut data = vec![
        0x6b, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96, 0xe9, 0x3d, 0x7e, 0x11, 0x73, 0x93, 0x17,
        0x2a,
    ];

    assert!(crypto_state::encrypt_data(h, &mut data));
    // Expected: f3eed1bdb5d2a03c064b5a7e3db181f8
    assert_eq!(data[0], 0xf3, "AES-256 NIST byte 0");
    assert_eq!(data[1], 0xee, "AES-256 NIST byte 1");
    assert_eq!(data[15], 0xf8, "AES-256 NIST byte 15");

    assert!(crypto_state::destroy_key(h));
}

#[test]
fn test_crypto_state_derive_key_from_hash() {
    let hh = helper::handler_create("test://derive_hash");
    assert!(crypto_state::create_hash(hh, CALG_MD5));
    assert!(crypto_state::hash_data(hh, b"password"));

    let hk = helper::handler_create("test://derive_key");
    assert!(crypto_state::derive_key_from_hash(hk, hh, CALG_AES_128));

    // Key should be the first 16 bytes of MD5("password") = 5f4dcc3b5aa765d61d8327deb882cf99
    let kp = crypto_state::get_key_param(hk, crypto_state::KP_KEYLEN).unwrap();
    let bits = u32::from_le_bytes([kp[0], kp[1], kp[2], kp[3]]);
    assert_eq!(bits, 128);

    // Verify the derived key works for encrypt/decrypt
    let original = [0u8; 16];
    let mut data = original.to_vec();
    assert!(crypto_state::encrypt_data(hk, &mut data));
    assert_ne!(&data[..], &original[..]);
    assert!(crypto_state::decrypt_data(hk, &mut data));
    assert_eq!(&data[..], &original[..]);

    assert!(crypto_state::destroy_hash(hh));
    assert!(crypto_state::destroy_key(hk));
}

// ═══════════════════════════════════════════════════════════════════════
//  crypto_state: multiple concurrent hashes
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_crypto_state_concurrent_hashes() {
    let h_md5 = helper::handler_create("test://concurrent_md5");
    let h_sha1 = helper::handler_create("test://concurrent_sha1");
    let h_sha256 = helper::handler_create("test://concurrent_sha256");

    assert!(crypto_state::create_hash(h_md5, CALG_MD5));
    assert!(crypto_state::create_hash(h_sha1, CALG_SHA1));
    assert!(crypto_state::create_hash(h_sha256, CALG_SHA_256));

    assert!(crypto_state::hash_data(h_md5, b"test"));
    assert!(crypto_state::hash_data(h_sha1, b"test"));
    assert!(crypto_state::hash_data(h_sha256, b"test"));

    let d_md5 = crypto_state::get_hash_param(h_md5, crypto_state::HP_HASHVAL).unwrap();
    let d_sha1 = crypto_state::get_hash_param(h_sha1, crypto_state::HP_HASHVAL).unwrap();
    let d_sha256 = crypto_state::get_hash_param(h_sha256, crypto_state::HP_HASHVAL).unwrap();

    assert_eq!(d_md5.len(), 16);
    assert_eq!(d_sha1.len(), 20);
    assert_eq!(d_sha256.len(), 32);

    // MD5("test") = 098f6bcd4621d373cade4e832627b4f6
    assert_eq!(d_md5[0], 0x09);
    // SHA1("test") = a94a8fe5ccb19ba61c4c0873d391e987982fbbd3
    assert_eq!(d_sha1[0], 0xa9);
    // SHA256("test") = 9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08
    assert_eq!(d_sha256[0], 0x9f);

    assert!(crypto_state::destroy_hash(h_md5));
    assert!(crypto_state::destroy_hash(h_sha1));
    assert!(crypto_state::destroy_hash(h_sha256));
}

// ═══════════════════════════════════════════════════════════════════════
//  Compression: LZNT1 pure function tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_lznt1_empty_stream() {
    // two zero bytes = terminator
    let input = [0x00u8, 0x00];
    let result = ntdll_common::lznt1_decompress(&input, 4096);
    assert!(result.is_some());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_lznt1_uncompressed_chunk() {
    // header = 0x0004 (uncompressed, 5 bytes: 5-1=4)
    // payload = "Hello"
    // terminator = 0x0000
    let input = [0x04, 0x00, b'H', b'e', b'l', b'l', b'o', 0x00, 0x00];
    let result = ntdll_common::lznt1_decompress(&input, 4096).unwrap();
    assert_eq!(&result, b"Hello");
}

#[test]
fn test_lznt1_uncompressed_chunk_single_byte() {
    // header = 0x0000 means 1 byte (0+1=1)... wait, 0x0000 is terminator
    // header = 0x0001 => uncompressed, 2 bytes
    // But for 1 byte: header = 0x0000 is the terminator. Let's use 2 bytes instead.
    let input = [0x01, 0x00, b'A', b'B', 0x00, 0x00];
    let result = ntdll_common::lznt1_decompress(&input, 4096).unwrap();
    assert_eq!(&result, b"AB");
}

#[test]
fn test_lznt1_compressed_all_literals() {
    // chunk header: 0x8008 (compressed, 9 bytes: 9-1=8)
    // flag byte: 0x00 (all 8 bits = literal)
    // 8 literal bytes
    let input = [
        0x08, 0x80, // compressed chunk, 9 bytes payload
        0x00, // flags: all literals
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', 0x00, 0x00, // terminator
    ];
    let result = ntdll_common::lznt1_decompress(&input, 4096).unwrap();
    assert_eq!(&result, b"ABCDEFGH");
}

#[test]
fn test_lznt1_compressed_with_backreference() {
    // Produce "AAAA": 1 literal 'A' + backreference(offset=1, length=3)
    // At output pos=1 within chunk, shift=12 (cur_pos <= 0x10)
    // tag: offset_field=(1-1)=0 << 12 = 0, length_field=(3-3)=0 → tag=0x0000
    // flag byte: 0x02 (bit0=literal, bit1=backref)
    // payload = flag(1) + literal(1) + tag(2) = 4 bytes
    // chunk header: 0x8003 (compressed, 4 bytes: 4-1=3)
    let input = [
        0x03, 0x80, // compressed chunk header
        0x02, // flags: bit0=literal, bit1=backreference
        b'A', // literal
        0x00, 0x00, // backreference tag
        0x00, 0x00, // terminator
    ];
    let result = ntdll_common::lznt1_decompress(&input, 4096).unwrap();
    assert_eq!(&result, b"AAAA");
}

#[test]
fn test_lznt1_two_uncompressed_chunks() {
    // First chunk: "AB" (header=0x0001)
    // Second chunk: "CD" (header=0x0001)
    // Terminator: 0x0000
    let input = [
        0x01, 0x00, b'A', b'B', // chunk 1
        0x01, 0x00, b'C', b'D', // chunk 2
        0x00, 0x00, // terminator
    ];
    let result = ntdll_common::lznt1_decompress(&input, 4096).unwrap();
    assert_eq!(&result, b"ABCD");
}

#[test]
fn test_lznt1_max_output_limit() {
    // Uncompressed chunk with 5 bytes, but limit to 3
    let input = [0x04, 0x00, b'H', b'e', b'l', b'l', b'o', 0x00, 0x00];
    let result = ntdll_common::lznt1_decompress(&input, 3);
    // Should fail because uncompressed chunk exceeds max_output
    assert!(result.is_none());
}

#[test]
fn test_lznt1_empty_input() {
    let result = ntdll_common::lznt1_decompress(&[], 4096);
    assert!(result.is_none());
}

#[test]
fn test_lznt1_truncated_header() {
    // Only 1 byte — can't read 2-byte header
    let result = ntdll_common::lznt1_decompress(&[0x01], 4096);
    // The loop condition is pos+2 <= len, so with 1 byte it exits cleanly
    assert!(result.is_some());
    assert!(result.unwrap().is_empty());
}

// ═══════════════════════════════════════════════════════════════════════
//  Compression: XPRESS plain pure function tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_xpress_all_literals() {
    // indicator = 0x00000000 (all 32 bits = literal)
    // 4 literal bytes
    let input = [0x00, 0x00, 0x00, 0x00, b'A', b'B', b'C', b'D'];
    let result = ntdll_common::xpress_decompress(&input, 4096).unwrap();
    assert_eq!(&result, b"ABCD");
}

#[test]
fn test_xpress_with_backreference() {
    // indicator = 0x00000002 (bit1 = match, bit0 = literal)
    // literal: 'A'
    // match hint: offset_field = (1-1)=0, shifted<<3 = 0; length_field = (3-3)=0
    // hint = 0x0000
    // Result: 'A' + copy(offset=1, len=3) = "AAAA"
    let input = [0x02, 0x00, 0x00, 0x00, b'A', 0x00, 0x00];
    let result = ntdll_common::xpress_decompress(&input, 4096).unwrap();
    assert_eq!(&result, b"AAAA");
}

#[test]
fn test_xpress_32_literals() {
    // indicator = 0x00000000
    // Exactly 32 literal bytes
    let mut input = vec![0x00, 0x00, 0x00, 0x00];
    input.extend_from_slice(&[b'X'; 32]);
    let result = ntdll_common::xpress_decompress(&input, 4096).unwrap();
    assert_eq!(result.len(), 32);
    assert!(result.iter().all(|&b| b == b'X'));
}

#[test]
fn test_xpress_short_input() {
    // Less than 4 bytes = can't even read the indicator
    let result = ntdll_common::xpress_decompress(&[0x00, 0x00], 4096);
    assert!(result.is_none());
}

#[test]
fn test_xpress_empty_input() {
    let result = ntdll_common::xpress_decompress(&[], 4096);
    assert!(result.is_none());
}

#[test]
fn test_xpress_max_output_limit() {
    // 32 literals but limit output to 4
    let mut input = vec![0x00, 0x00, 0x00, 0x00];
    input.extend_from_slice(&[b'Z'; 32]);
    let result = ntdll_common::xpress_decompress(&input, 4).unwrap();
    assert_eq!(result.len(), 4);
    assert!(result.iter().all(|&b| b == b'Z'));
}

#[test]
fn test_xpress_match_with_extra_length_byte() {
    // indicator: bit0=literal, bit1=match
    // literal: 'B'
    // match hint: length_field=7 (triggers extra byte read)
    //   offset_field = (1-1)=0, hint = (0<<3)|7 = 0x0007
    //   extra byte = 3 → total length = 3 + 7 + 3 = 13
    // Result: 'B' + 13 copies = 14 'B's
    let input = [
        0x02, 0x00, 0x00, 0x00, // indicator: bit1=match
        b'B', // literal
        0x07, 0x00, // match hint: len=7, offset=1
        3,    // extra length byte
    ];
    let result = ntdll_common::xpress_decompress(&input, 4096).unwrap();
    assert_eq!(result.len(), 14);
    assert!(result.iter().all(|&b| b == b'B'));
}

// ═══════════════════════════════════════════════════════════════════════
//  Compression: XPRESS+Huffman pure function tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_xpress_huff_empty_input() {
    let result = ntdll_common::xpress_huff_decompress(&[], 4096);
    assert!(result.is_none());
}

#[test]
fn test_xpress_huff_short_table() {
    // Less than 256 bytes = can't read the Huffman table
    let input = [0u8; 100];
    let result = ntdll_common::xpress_huff_decompress(&input, 4096);
    assert!(result.is_none());
}

#[test]
fn test_xpress_huff_all_zero_table() {
    // 256-byte table of zeros = all symbols have length 0 → no valid codes
    // + some data bytes
    let input = vec![0u8; 256 + 16];
    let result = ntdll_common::xpress_huff_decompress(&input, 4096);
    // With all zero lengths, the decode table has nbits=0 everywhere, causing immediate break
    // returns None because output is empty
    assert!(result.is_none());
}

#[test]
fn test_xpress_huff_single_symbol_stream() {
    // Build a Huffman table where only symbol 0x41 ('A') has a code,
    // with bit length 1. All other symbols have length 0.
    //
    // The 512 symbols are packed: table[i] = lens[2*i] | (lens[2*i+1] << 4)
    // Symbol 0x41 = 65 decimal → table byte index = 65/2 = 32, is odd nibble
    // So table[32] = 0x10 (low nibble=0 for sym 64, high nibble=1 for sym 65)
    let mut table = [0u8; 256];
    table[32] = 0x10; // symbol 65 ('A') gets bit length 1

    // Bitstream: all 0 bits → code 0 → this decodes to whatever code=0 maps to.
    // With only symbol 65 having length 1, code 0 reversed = 0, fill_count = 2^14,
    // so table indices 0..16383 all decode to symbol 65.
    // But code 0 bit-reversed for len=1 is 0, and base = 0 << 14 = 0.
    // fill_count = 1<<14 = 16384. So entries 0..16383 = 65 | (1<<9) = 65+512 = 577 = 0x0241.
    //
    // The bitstream is pairs of bytes interpreted as u16 LE. Each refill reads 2 bytes.
    // Let's provide 8 bytes of zeros as the bitstream.
    let mut input = Vec::new();
    input.extend_from_slice(&table);
    input.extend_from_slice(&[0u8; 8]); // bitstream: all zeros

    let result = ntdll_common::xpress_huff_decompress(&input, 10);
    assert!(result.is_some());
    let out = result.unwrap();
    assert!(!out.is_empty());
    assert!(
        out.iter().all(|&b| b == b'A'),
        "all decoded bytes should be 'A'"
    );
}

// ═══════════════════════════════════════════════════════════════════════
//  Compression: protect_to_rwx
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_protect_to_rwx_readonly() {
    let (r, w, x) = ntdll_common::protect_to_rwx(0x02); // PAGE_READONLY
    assert!(r);
    assert!(!w);
    assert!(!x);
}

#[test]
fn test_protect_to_rwx_readwrite() {
    let (r, w, x) = ntdll_common::protect_to_rwx(0x04); // PAGE_READWRITE
    assert!(r);
    assert!(w);
    assert!(!x);
}

#[test]
fn test_protect_to_rwx_execute_read() {
    let (r, w, x) = ntdll_common::protect_to_rwx(0x20); // PAGE_EXECUTE_READ
    assert!(r);
    assert!(!w);
    assert!(x);
}

#[test]
fn test_protect_to_rwx_execute_readwrite() {
    let (r, w, x) = ntdll_common::protect_to_rwx(0x40); // PAGE_EXECUTE_READWRITE
    assert!(r);
    assert!(w);
    assert!(x);
}

#[test]
fn test_protect_to_rwx_execute_only() {
    let (r, w, x) = ntdll_common::protect_to_rwx(0x10); // PAGE_EXECUTE
    assert!(!r);
    assert!(!w);
    assert!(x);
}

#[test]
fn test_protect_to_rwx_none() {
    let (r, w, x) = ntdll_common::protect_to_rwx(0x00);
    assert!(!r);
    assert!(!w);
    assert!(!x);
}

#[test]
fn test_protect_to_rwx_writecopy() {
    let (r, w, x) = ntdll_common::protect_to_rwx(0x08); // PAGE_WRITECOPY
    assert!(r);
    assert!(w);
    assert!(!x);
}

#[test]
fn test_protect_to_rwx_execute_writecopy() {
    let (r, w, x) = ntdll_common::protect_to_rwx(0x80); // PAGE_EXECUTE_WRITECOPY
    assert!(r);
    assert!(w);
    assert!(x);
}

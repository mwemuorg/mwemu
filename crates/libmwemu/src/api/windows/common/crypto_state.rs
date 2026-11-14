use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
use aes::{Aes128, Aes256};
use des::{Des, TdesEde3};
use sha1::Sha1;
use sha2::{Digest as Sha2Digest, Sha256};
use std::collections::HashMap;
use std::sync::Mutex;

const CALG_SHA_256: u32 = 0x0000800c;
const CALG_SHA1: u32 = 0x00008004;
const CALG_MD5: u32 = 0x00008003;
const CALG_RC4: u32 = 0x00006801;
const CALG_AES_128: u32 = 0x0000660e;
const CALG_AES_256: u32 = 0x00006610;
const CALG_DES: u32 = 0x00006601;
const CALG_3DES: u32 = 0x00006603;

pub const HP_HASHVAL: u32 = 0x0002;
pub const HP_HASHSIZE: u32 = 0x0004;
pub const HP_ALGID: u32 = 0x0001;

pub const KP_BLOCKLEN: u32 = 0x0008;
pub const KP_KEYLEN: u32 = 0x0007;
pub const KP_ALGID: u32 = 0x0001;

enum HashState {
    Md5(md5::Context),
    Sha1(Sha1),
    Sha256(Sha256),
}

struct CryptoHash {
    alg_id: u32,
    state: HashState,
    finalized: Option<Vec<u8>>,
}

struct CryptoKey {
    alg_id: u32,
    key_data: Vec<u8>,
}

struct CryptoStore {
    hashes: HashMap<u64, CryptoHash>,
    keys: HashMap<u64, CryptoKey>,
}

static STORE: Mutex<Option<CryptoStore>> = Mutex::new(None);

fn with_store<F, R>(f: F) -> R
where
    F: FnOnce(&mut CryptoStore) -> R,
{
    let mut guard = STORE.lock().unwrap_or_else(|e| e.into_inner());
    if guard.is_none() {
        *guard = Some(CryptoStore {
            hashes: HashMap::new(),
            keys: HashMap::new(),
        });
    }
    f(guard.as_mut().unwrap())
}

pub fn create_hash(handle: u64, alg_id: u32) -> bool {
    let state = match alg_id {
        CALG_MD5 => HashState::Md5(md5::Context::new()),
        CALG_SHA1 => HashState::Sha1(Sha1::new()),
        CALG_SHA_256 => HashState::Sha256(Sha256::new()),
        _ => return false,
    };
    with_store(|s| {
        s.hashes.insert(
            handle,
            CryptoHash {
                alg_id,
                state,
                finalized: None,
            },
        );
    });
    true
}

pub fn hash_data(handle: u64, data: &[u8]) -> bool {
    with_store(|s| {
        if let Some(h) = s.hashes.get_mut(&handle) {
            if h.finalized.is_some() {
                return false;
            }
            match &mut h.state {
                HashState::Md5(ctx) => ctx.consume(data),
                HashState::Sha1(hasher) => {
                    use sha1::Digest;
                    hasher.update(data);
                }
                HashState::Sha256(hasher) => hasher.update(data),
            }
            true
        } else {
            false
        }
    })
}

pub fn get_hash_param(handle: u64, param: u32) -> Option<Vec<u8>> {
    with_store(|s| {
        let h = s.hashes.get_mut(&handle)?;
        match param {
            HP_HASHVAL => {
                if h.finalized.is_none() {
                    let digest = match &h.state {
                        HashState::Md5(ctx) => {
                            let result = ctx.clone().compute();
                            result.to_vec()
                        }
                        HashState::Sha1(hasher) => {
                            use sha1::Digest;
                            let result = hasher.clone().finalize();
                            result.to_vec()
                        }
                        HashState::Sha256(hasher) => {
                            let result = hasher.clone().finalize();
                            result.to_vec()
                        }
                    };
                    h.finalized = Some(digest);
                }
                h.finalized.clone()
            }
            HP_HASHSIZE => {
                let size: u32 = match h.alg_id {
                    CALG_MD5 => 16,
                    CALG_SHA1 => 20,
                    CALG_SHA_256 => 32,
                    _ => 0,
                };
                Some(size.to_le_bytes().to_vec())
            }
            HP_ALGID => Some(h.alg_id.to_le_bytes().to_vec()),
            _ => None,
        }
    })
}

pub fn destroy_hash(handle: u64) -> bool {
    with_store(|s| s.hashes.remove(&handle).is_some())
}

pub fn create_key(handle: u64, alg_id: u32, key_len_bits: u32) -> bool {
    let key_len = (key_len_bits as usize + 7) / 8;
    let key_len = if key_len == 0 {
        match alg_id {
            CALG_RC4 => 16,
            CALG_AES_128 => 16,
            CALG_AES_256 => 32,
            _ => 16,
        }
    } else {
        key_len
    };
    let key_data: Vec<u8> = (0..key_len).map(|i| (i as u8).wrapping_mul(0x37)).collect();
    with_store(|s| {
        s.keys.insert(handle, CryptoKey { alg_id, key_data });
    });
    true
}

pub fn derive_key_from_hash(key_handle: u64, hash_handle: u64, alg_id: u32) -> bool {
    let digest = match get_hash_param(hash_handle, HP_HASHVAL) {
        Some(d) => d,
        None => return false,
    };
    let key_len = match alg_id {
        CALG_RC4 => digest.len().min(256),
        CALG_AES_128 => 16,
        CALG_AES_256 => 32,
        CALG_DES => 8,
        CALG_3DES => 24,
        _ => digest.len().min(32),
    };
    let mut key_data = vec![0u8; key_len];
    for (i, b) in key_data.iter_mut().enumerate() {
        *b = if i < digest.len() { digest[i] } else { 0 };
    }
    with_store(|s| {
        s.keys.insert(key_handle, CryptoKey { alg_id, key_data });
    });
    true
}

pub fn set_key_data(handle: u64, data: &[u8]) -> bool {
    with_store(|s| {
        if let Some(k) = s.keys.get_mut(&handle) {
            k.key_data = data.to_vec();
            true
        } else {
            false
        }
    })
}

pub fn get_key_param(handle: u64, param: u32) -> Option<Vec<u8>> {
    with_store(|s| {
        let k = s.keys.get(&handle)?;
        match param {
            KP_KEYLEN => {
                let bits = (k.key_data.len() * 8) as u32;
                Some(bits.to_le_bytes().to_vec())
            }
            KP_BLOCKLEN => {
                let block: u32 = match k.alg_id {
                    CALG_AES_128 | CALG_AES_256 => 128,
                    CALG_RC4 => 0,
                    _ => 64,
                };
                Some(block.to_le_bytes().to_vec())
            }
            KP_ALGID => Some(k.alg_id.to_le_bytes().to_vec()),
            _ => None,
        }
    })
}

fn rc4_crypt(key: &[u8], data: &mut [u8]) {
    let mut s: Vec<u8> = (0..=255u8).collect();
    let mut j: u8 = 0;
    for i in 0..256 {
        j = j.wrapping_add(s[i]).wrapping_add(key[i % key.len()]);
        s.swap(i, j as usize);
    }
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    for byte in data.iter_mut() {
        i = i.wrapping_add(1);
        j = j.wrapping_add(s[i as usize]);
        s.swap(i as usize, j as usize);
        let k = s[(s[i as usize].wrapping_add(s[j as usize])) as usize];
        *byte ^= k;
    }
}

fn des_encrypt_ecb(key: &[u8], data: &mut [u8]) {
    if key.len() < 8 {
        return;
    }
    if let Ok(cipher) = Des::new_from_slice(&key[..8]) {
        for chunk in data.chunks_mut(8) {
            if chunk.len() == 8 {
                let block = des::cipher::Block::<Des>::from_mut_slice(chunk);
                cipher.encrypt_block(block);
            }
        }
    }
}

fn des_decrypt_ecb(key: &[u8], data: &mut [u8]) {
    if key.len() < 8 {
        return;
    }
    if let Ok(cipher) = Des::new_from_slice(&key[..8]) {
        for chunk in data.chunks_mut(8) {
            if chunk.len() == 8 {
                let block = des::cipher::Block::<Des>::from_mut_slice(chunk);
                cipher.decrypt_block(block);
            }
        }
    }
}

fn tdes_encrypt_ecb(key: &[u8], data: &mut [u8]) {
    if key.len() < 24 {
        return;
    }
    if let Ok(cipher) = TdesEde3::new_from_slice(&key[..24]) {
        for chunk in data.chunks_mut(8) {
            if chunk.len() == 8 {
                let block = des::cipher::Block::<TdesEde3>::from_mut_slice(chunk);
                cipher.encrypt_block(block);
            }
        }
    }
}

fn tdes_decrypt_ecb(key: &[u8], data: &mut [u8]) {
    if key.len() < 24 {
        return;
    }
    if let Ok(cipher) = TdesEde3::new_from_slice(&key[..24]) {
        for chunk in data.chunks_mut(8) {
            if chunk.len() == 8 {
                let block = des::cipher::Block::<TdesEde3>::from_mut_slice(chunk);
                cipher.decrypt_block(block);
            }
        }
    }
}

fn aes128_encrypt_ecb(key: &[u8], data: &mut [u8]) {
    if key.len() < 16 {
        return;
    }
    let cipher = Aes128::new_from_slice(&key[..16]).unwrap();
    for chunk in data.chunks_mut(16) {
        if chunk.len() == 16 {
            let block = aes::Block::from_mut_slice(chunk);
            cipher.encrypt_block(block);
        }
    }
}

fn aes128_decrypt_ecb(key: &[u8], data: &mut [u8]) {
    if key.len() < 16 {
        return;
    }
    let cipher = Aes128::new_from_slice(&key[..16]).unwrap();
    for chunk in data.chunks_mut(16) {
        if chunk.len() == 16 {
            let block = aes::Block::from_mut_slice(chunk);
            cipher.decrypt_block(block);
        }
    }
}

fn aes256_encrypt_ecb(key: &[u8], data: &mut [u8]) {
    if key.len() < 32 {
        return;
    }
    let cipher = Aes256::new_from_slice(&key[..32]).unwrap();
    for chunk in data.chunks_mut(16) {
        if chunk.len() == 16 {
            let block = aes::Block::from_mut_slice(chunk);
            cipher.encrypt_block(block);
        }
    }
}

fn aes256_decrypt_ecb(key: &[u8], data: &mut [u8]) {
    if key.len() < 32 {
        return;
    }
    let cipher = Aes256::new_from_slice(&key[..32]).unwrap();
    for chunk in data.chunks_mut(16) {
        if chunk.len() == 16 {
            let block = aes::Block::from_mut_slice(chunk);
            cipher.decrypt_block(block);
        }
    }
}

pub fn encrypt_data(handle: u64, data: &mut [u8]) -> bool {
    with_store(|s| {
        if let Some(k) = s.keys.get(&handle) {
            match k.alg_id {
                CALG_RC4 => rc4_crypt(&k.key_data, data),
                CALG_DES => des_encrypt_ecb(&k.key_data, data),
                CALG_3DES => tdes_encrypt_ecb(&k.key_data, data),
                CALG_AES_128 => aes128_encrypt_ecb(&k.key_data, data),
                CALG_AES_256 => aes256_encrypt_ecb(&k.key_data, data),
                _ => {
                    for (i, b) in data.iter_mut().enumerate() {
                        *b ^= k.key_data[i % k.key_data.len()];
                    }
                }
            }
            true
        } else {
            false
        }
    })
}

pub fn decrypt_data(handle: u64, data: &mut [u8]) -> bool {
    with_store(|s| {
        if let Some(k) = s.keys.get(&handle) {
            match k.alg_id {
                CALG_RC4 => rc4_crypt(&k.key_data, data),
                CALG_DES => des_decrypt_ecb(&k.key_data, data),
                CALG_3DES => tdes_decrypt_ecb(&k.key_data, data),
                CALG_AES_128 => aes128_decrypt_ecb(&k.key_data, data),
                CALG_AES_256 => aes256_decrypt_ecb(&k.key_data, data),
                _ => {
                    for (i, b) in data.iter_mut().enumerate() {
                        *b ^= k.key_data[i % k.key_data.len()];
                    }
                }
            }
            true
        } else {
            false
        }
    })
}

pub const KP_IV: u32 = 0x0001;
pub const KP_MODE: u32 = 0x0004;

pub fn set_key_param(handle: u64, param: u32, data: &[u8]) -> bool {
    with_store(|s| {
        if let Some(_k) = s.keys.get_mut(&handle) {
            match param {
                KP_IV | KP_MODE => true,
                _ => true,
            }
        } else {
            false
        }
    })
}

pub fn export_key(handle: u64) -> Option<Vec<u8>> {
    with_store(|s| {
        let k = s.keys.get(&handle)?;
        Some(k.key_data.clone())
    })
}

pub fn destroy_key(handle: u64) -> bool {
    with_store(|s| s.keys.remove(&handle).is_some())
}

const DPAPI_KEY: [u8; 32] = [
    0x4d, 0x57, 0x45, 0x4d, 0x55, 0x5f, 0x44, 0x50, 0x41, 0x50, 0x49, 0x5f, 0x4b, 0x45, 0x59, 0x5f,
    0x32, 0x35, 0x36, 0x5f, 0x42, 0x49, 0x54, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x49, 0x43, 0x21, 0x00,
];

pub fn dpapi_protect(data: &[u8]) -> Vec<u8> {
    let padded_len = ((data.len() + 15) / 16) * 16;
    let mut padded = vec![0u8; padded_len + 4];
    padded[..4].copy_from_slice(&(data.len() as u32).to_le_bytes());
    padded[4..4 + data.len()].copy_from_slice(data);
    let cipher = Aes256::new_from_slice(&DPAPI_KEY).unwrap();
    for chunk in padded[4..].chunks_mut(16) {
        if chunk.len() == 16 {
            let block = aes::Block::from_mut_slice(chunk);
            cipher.encrypt_block(block);
        }
    }
    padded
}

pub fn dpapi_unprotect(data: &[u8]) -> Vec<u8> {
    if data.len() < 20 {
        return data.to_vec();
    }
    let orig_len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
    let mut decrypted = data[4..].to_vec();
    let cipher = Aes256::new_from_slice(&DPAPI_KEY).unwrap();
    for chunk in decrypted.chunks_mut(16) {
        if chunk.len() == 16 {
            let block = aes::Block::from_mut_slice(chunk);
            cipher.decrypt_block(block);
        }
    }
    let safe_len = orig_len.min(decrypted.len());
    decrypted.truncate(safe_len);
    decrypted
}

pub fn sign_hash(hash_handle: u64, sig_len: usize) -> Option<Vec<u8>> {
    let hash_val = get_hash_param(hash_handle, HP_HASHVAL)?;
    let alg_id = with_store(|s| s.hashes.get(&hash_handle).map(|h| h.alg_id))?;
    let digest_info_prefix: &[u8] = match alg_id {
        CALG_MD5 => &[
            0x30, 0x20, 0x30, 0x0c, 0x06, 0x08, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x02, 0x05,
            0x05, 0x00, 0x04, 0x10,
        ],
        CALG_SHA1 => &[
            0x30, 0x21, 0x30, 0x09, 0x06, 0x05, 0x2b, 0x0e, 0x03, 0x02, 0x1a, 0x05, 0x00, 0x04,
            0x14,
        ],
        CALG_SHA_256 => &[
            0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
            0x01, 0x05, 0x00, 0x04, 0x20,
        ],
        _ => &[],
    };
    let t_len = digest_info_prefix.len() + hash_val.len();
    let out_len = if sig_len > 0 { sig_len } else { 128 };
    if out_len < t_len + 11 {
        return Some(hash_val);
    }
    let ps_len = out_len - t_len - 3;
    let mut sig = vec![0u8; out_len];
    sig[0] = 0x00;
    sig[1] = 0x01;
    for b in &mut sig[2..2 + ps_len] {
        *b = 0xff;
    }
    sig[2 + ps_len] = 0x00;
    sig[3 + ps_len..3 + ps_len + digest_info_prefix.len()].copy_from_slice(digest_info_prefix);
    sig[3 + ps_len + digest_info_prefix.len()..].copy_from_slice(&hash_val);
    Some(sig)
}

pub fn pbkdf2_derive(alg_id: u32, password: &[u8], salt: &[u8], iterations: u32, out: &mut [u8]) {
    match alg_id {
        CALG_SHA1 => {
            pbkdf2::pbkdf2_hmac::<Sha1>(password, salt, iterations, out);
        }
        _ => {
            pbkdf2::pbkdf2_hmac::<Sha256>(password, salt, iterations, out);
        }
    }
}

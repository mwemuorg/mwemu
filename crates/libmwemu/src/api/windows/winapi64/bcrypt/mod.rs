use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64::kernel32;

mod process_prng;
pub use process_prng::ProcessPrng;
mod b_crypt_close_algorithm_provider;
pub use b_crypt_close_algorithm_provider::BCryptCloseAlgorithmProvider;
mod b_crypt_create_hash;
pub use b_crypt_create_hash::BCryptCreateHash;
mod b_crypt_decrypt;
pub use b_crypt_decrypt::BCryptDecrypt;
mod b_crypt_derive_key_pbkdf2;
pub use b_crypt_derive_key_pbkdf2::BCryptDeriveKeyPBKDF2;
mod b_crypt_destroy_hash;
pub use b_crypt_destroy_hash::BCryptDestroyHash;
mod b_crypt_destroy_key;
pub use b_crypt_destroy_key::BCryptDestroyKey;
mod b_crypt_encrypt;
pub use b_crypt_encrypt::BCryptEncrypt;
mod b_crypt_export_key;
pub use b_crypt_export_key::BCryptExportKey;
mod b_crypt_finalize_key_pair;
pub use b_crypt_finalize_key_pair::BCryptFinalizeKeyPair;
mod b_crypt_finish_hash;
pub use b_crypt_finish_hash::BCryptFinishHash;
mod b_crypt_gen_random;
pub use b_crypt_gen_random::BCryptGenRandom;
mod b_crypt_generate_key_pair;
pub use b_crypt_generate_key_pair::BCryptGenerateKeyPair;
mod b_crypt_generate_symmetric_key;
pub use b_crypt_generate_symmetric_key::BCryptGenerateSymmetricKey;
mod b_crypt_get_property;
pub use b_crypt_get_property::BCryptGetProperty;
mod b_crypt_hash;
pub use b_crypt_hash::BCryptHash;
mod b_crypt_hash_data;
pub use b_crypt_hash_data::BCryptHashData;
mod b_crypt_import_key_pair;
pub use b_crypt_import_key_pair::BCryptImportKeyPair;
mod b_crypt_open_algorithm_provider;
pub use b_crypt_open_algorithm_provider::BCryptOpenAlgorithmProvider;
mod b_crypt_set_property;
pub use b_crypt_set_property::BCryptSetProperty;
mod b_crypt_sign_hash;
pub use b_crypt_sign_hash::BCryptSignHash;
mod b_crypt_verify_signature;
pub use b_crypt_verify_signature::BCryptVerifySignature;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "BCryptOpenAlgorithmProvider" => BCryptOpenAlgorithmProvider(emu),
        "BCryptGenRandom" => BCryptGenRandom(emu),
        "BCryptCloseAlgorithmProvider" => BCryptCloseAlgorithmProvider(emu),
        "BCryptCreateHash" => BCryptCreateHash(emu),
        "BCryptHashData" => BCryptHashData(emu),
        "BCryptFinishHash" => BCryptFinishHash(emu),
        "BCryptDestroyHash" => BCryptDestroyHash(emu),
        "BCryptGetProperty" => BCryptGetProperty(emu),
        "BCryptDeriveKeyPBKDF2" => BCryptDeriveKeyPBKDF2(emu),
        "BCryptGenerateSymmetricKey" => BCryptGenerateSymmetricKey(emu),
        "BCryptDestroyKey" => BCryptDestroyKey(emu),
        "BCryptEncrypt" => BCryptEncrypt(emu),
        "BCryptDecrypt" => BCryptDecrypt(emu),
        "BCryptSetProperty" => BCryptSetProperty(emu),
        "BCryptImportKeyPair" => BCryptImportKeyPair(emu),
        "BCryptExportKey" => BCryptExportKey(emu),
        "BCryptSignHash" => BCryptSignHash(emu),
        "BCryptVerifySignature" => BCryptVerifySignature(emu),
        "BCryptGenerateKeyPair" => BCryptGenerateKeyPair(emu),
        "BCryptFinalizeKeyPair" => BCryptFinalizeKeyPair(emu),
        "BCryptHash" => BCryptHash(emu),
        "ProcessPrng" => ProcessPrng(emu),
        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}

fn bcrypt_algo_to_calg(name: &str) -> u32 {
    match name.to_uppercase().as_str() {
        "SHA256" => 0x0000800c,
        "SHA1" => 0x00008004,
        "MD5" => 0x00008003,
        _ => 0,
    }
}

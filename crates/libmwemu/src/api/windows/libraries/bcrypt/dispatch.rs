use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::libraries::bcrypt::exports;

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "BCryptOpenAlgorithmProvider" => exports::BCryptOpenAlgorithmProvider(emu),
        "BCryptGenRandom" => exports::BCryptGenRandom(emu),
        "BCryptCloseAlgorithmProvider" => exports::BCryptCloseAlgorithmProvider(emu),
        "BCryptCreateHash" => exports::BCryptCreateHash(emu),
        "BCryptHashData" => exports::BCryptHashData(emu),
        "BCryptFinishHash" => exports::BCryptFinishHash(emu),
        "BCryptDestroyHash" => exports::BCryptDestroyHash(emu),
        "BCryptGetProperty" => exports::BCryptGetProperty(emu),
        "BCryptDeriveKeyPBKDF2" => exports::BCryptDeriveKeyPBKDF2(emu),
        "BCryptGenerateSymmetricKey" => exports::BCryptGenerateSymmetricKey(emu),
        "BCryptDestroyKey" => exports::BCryptDestroyKey(emu),
        "BCryptEncrypt" => exports::BCryptEncrypt(emu),
        "BCryptDecrypt" => exports::BCryptDecrypt(emu),
        "BCryptSetProperty" => exports::BCryptSetProperty(emu),
        "BCryptImportKeyPair" => exports::BCryptImportKeyPair(emu),
        "BCryptExportKey" => exports::BCryptExportKey(emu),
        "BCryptSignHash" => exports::BCryptSignHash(emu),
        "BCryptVerifySignature" => exports::BCryptVerifySignature(emu),
        "BCryptGenerateKeyPair" => exports::BCryptGenerateKeyPair(emu),
        "BCryptFinalizeKeyPair" => exports::BCryptFinalizeKeyPair(emu),
        "BCryptHash" => exports::BCryptHash(emu),
        "ProcessPrng" => exports::ProcessPrng(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

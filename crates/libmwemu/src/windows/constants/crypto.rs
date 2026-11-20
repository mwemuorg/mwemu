pub const PROV_RSA_FULL: u32 = 1;

pub const HP_ALGID: u32 = 1;
pub const HP_HASHVAL: u32 = 2;
pub const HP_HASHSIZE: u32 = 4;

pub const KP_ALGID: u32 = 7;
pub const KP_BLOCKLEN: u32 = 8;
pub const KP_KEYLEN: u32 = 9;

pub const COMPRESSION_FORMAT_LZNT1: u16 = 2;
pub const COMPRESSION_ENGINE_MAXIMUM: u16 = 0x0100;
pub const COMPRESSION_FORMAT_XPRESS: u16 = 0x0003;
pub const COMPRESSION_FORMAT_XPRESS_HUFF: u16 = 0x0004;

pub const CERT_STORE_PROV_MEMORY: u64 = 2;
pub const CERT_STORE_PROV_SYSTEM_A: u64 = 9;
pub const CERT_STORE_PROV_SYSTEM_W: u64 = 10;

pub const X509_ASN_ENCODING: u32 = 1;
pub const PKCS_7_ASN_ENCODING: u32 = 0x0001_0000;

pub const CRYPT_STRING_NOCRLF: u64 = 0x4000_0000;
pub const CRYPT_STRING_NOCR: u64 = 0x8000_0000;
pub const CRYPT_STRING_BASE64: u64 = 1;
pub const CRYPT_STRING_HEX: u64 = 4;
pub const CRYPT_STRING_BASE64HEADER: u64 = 0x00000000;
pub const CRYPT_STRING_BINARY: u64 = 0x00000002;
pub const CRYPT_STRING_BASE64_ANY: u64 = 0x00000006;
pub const CRYPT_STRING_ANY: u64 = 0x00000007;
pub const CRYPT_STRING_HEX_ANY: u64 = 0x00000008;
pub const CRYPT_STRING_HEXRAW: u64 = 0x0000000c;

// CryptAcquireContext flags
pub const CRYPT_MACHINE_DEFAULT: u32 = 1;
pub const CRYPT_USER_DEFAULT: u32 = 2;
pub const CRYPT_EXPORTABLE: u32 = 1;
pub const PLAINTEXTKEYBLOB: u8 = 8;
pub const PP_NAME: u32 = 4;
pub const PP_PROVTYPE: u32 = 16;

pub const CRYPT_VERIFYCONTEXT: u32 = 0xF0000000;
pub const CRYPT_NEWKEYSET: u32 = 0x00000008;
pub const CRYPT_DELETEKEYSET: u32 = 0x00000010;
pub const CRYPT_MACHINE_KEYSET: u32 = 0x00000020;
pub const CRYPT_SILENT: u32 = 0x00000040;
pub const CRYPT_DEFAULT_CONTAINER_OPTIONAL: u32 = 0x00000080;

// CryptoAPI algorithm identifiers
pub const CALG_3DES: u32 = 0x00006603;
pub const CALG_3DES_112: u32 = 0x00006609;
pub const CALG_AES: u32 = 0x00006611;
pub const CALG_AES_128: u32 = 0x0000660e;
pub const CALG_AES_192: u32 = 0x0000660f;
pub const CALG_AES_256: u32 = 0x00006610;
pub const CALG_AGREEDKEY_ANY: u32 = 0x0000aa03;
pub const CALG_CYLINK_MEK: u32 = 0x0000660c;
pub const CALG_DES: u32 = 0x00006601;
pub const CALG_DESX: u32 = 0x00006604;
pub const CALG_DH_EPHEM: u32 = 0x0000aa02;
pub const CALG_DH_SF: u32 = 0x0000aa01;
pub const CALG_DSS_SIGN: u32 = 0x00002200;
pub const CALG_ECDH: u32 = 0x0000aa05;
pub const CALG_ECDH_EPHEM: u32 = 0x0000ae06;
pub const CALG_ECDSA: u32 = 0x00002203;
pub const CALG_ECMQV: u32 = 0x0000a001;
pub const CALG_HASH_REPLACE_OWF: u32 = 0x0000800b;
pub const CALG_HUGHES_MD5: u32 = 0x0000a003;
pub const CALG_HMAC: u32 = 0x00008009;
pub const CALG_KEA_KEYX: u32 = 0x0000aa04;
pub const CALG_MAC: u32 = 0x00008005;
pub const CALG_MD2: u32 = 0x00008001;
pub const CALG_MD4: u32 = 0x00008002;
pub const CALG_MD5: u32 = 0x00008003;
pub const CALG_NO_SIGN: u32 = 0x00002000;
pub const CALG_OID_INFO_CNG_ONLY: u32 = 0xffffffff;
pub const CALG_OID_INFO_PARAMETERS: u32 = 0xfffffffe;
pub const CALG_PCT1_MASTER: u32 = 0x00004c04;
pub const CALG_RC2: u32 = 0x00006602;
pub const CALG_RC4: u32 = 0x00006801;
pub const CALG_RC5: u32 = 0x0000660d;
pub const CALG_RSA_KEYX: u32 = 0x0000a400;
pub const CALG_RSA_SIGN: u32 = 0x00002400;
pub const CALG_SCHANNEL_ENC_KEY: u32 = 0x00004c07;
pub const CALG_SCHANNEL_MAC_KEY: u32 = 0x00004c03;
pub const CALG_SCHANNEL_MASTER_HASH: u32 = 0x00004c02;
pub const CALG_SEAL: u32 = 0x00006802;
pub const CALG_SHA: u32 = 0x00008004;
pub const CALG_SHA1: u32 = 0x00008004;
pub const CALG_SHA_256: u32 = 0x0000800c;
pub const CALG_SHA_384: u32 = 0x0000800d;
pub const CALG_SHA_512: u32 = 0x0000800e;
pub const CALG_SKIPJACK: u32 = 0x0000660a;
pub const CALG_SSL2_MASTER: u32 = 0x00004c05;
pub const CALG_SSL3_MASTER: u32 = 0x00004c01;
pub const CALG_SSL3_SHAMD5: u32 = 0x00008008;
pub const CALG_TEK: u32 = 0x0000660b;
pub const CALG_TLS1_MASTER: u32 = 0x00004c06;
pub const CALG_TLS1PRF: u32 = 0x0000800a;

pub fn get_cryptoalgorithm_name(value: u32) -> &'static str {
    match value {
        CALG_3DES => "CALG_3DES",
        CALG_3DES_112 => "CALG_3DES_112",
        CALG_AES => "CALG_AES",
        CALG_AES_128 => "CALG_AES_128",
        CALG_AES_192 => "CALG_AES_192",
        CALG_AES_256 => "CALG_AES_256",
        CALG_AGREEDKEY_ANY => "CALG_AGREEDKEY_ANY",
        CALG_CYLINK_MEK => "CALG_CYLINK_MEK",
        CALG_DES => "CALG_DES",
        CALG_DESX => "CALG_DESX",
        CALG_DH_EPHEM => "CALG_DH_EPHEM",
        CALG_DH_SF => "CALG_DH_SF",
        CALG_DSS_SIGN => "CALG_DSS_SIGN",
        CALG_ECDH => "CALG_ECDH",
        CALG_ECDH_EPHEM => "CALG_ECDH_EPHEM",
        CALG_ECDSA => "CALG_ECDSA",
        CALG_ECMQV => "CALG_ECMQV",
        CALG_HASH_REPLACE_OWF => "CALG_HASH_REPLACE_OWF",
        CALG_HUGHES_MD5 => "CALG_HUGHES_MD5",
        CALG_HMAC => "CALG_HMAC",
        CALG_KEA_KEYX => "CALG_KEA_KEYX",
        CALG_MAC => "CALG_MAC",
        CALG_MD2 => "CALG_MD2",
        CALG_MD4 => "CALG_MD4",
        CALG_MD5 => "CALG_MD5",
        CALG_NO_SIGN => "CALG_NO_SIGN",
        CALG_OID_INFO_CNG_ONLY => "CALG_OID_INFO_CNG_ONLY",
        CALG_OID_INFO_PARAMETERS => "CALG_OID_INFO_PARAMETERS",
        CALG_PCT1_MASTER => "CALG_PCT1_MASTER",
        CALG_RC2 => "CALG_RC2",
        CALG_RC4 => "CALG_RC4",
        CALG_RC5 => "CALG_RC5",
        CALG_RSA_KEYX => "CALG_RSA_KEYX",
        CALG_RSA_SIGN => "CALG_RSA_SIGN",
        CALG_SCHANNEL_ENC_KEY => "CALG_SCHANNEL_ENC_KEY",
        CALG_SCHANNEL_MAC_KEY => "CALG_SCHANNEL_MAC_KEY",
        CALG_SCHANNEL_MASTER_HASH => "CALG_SCHANNEL_MASTER_HASH",
        CALG_SEAL => "CALG_SEAL",
        CALG_SHA => "CALG_SHA",
        CALG_SHA_256 => "CALG_SHA_256",
        CALG_SHA_384 => "CALG_SHA_384",
        CALG_SHA_512 => "CALG_SHA_512",
        CALG_SKIPJACK => "CALG_SKIPJACK",
        CALG_SSL2_MASTER => "CALG_SSL2_MASTER",
        CALG_SSL3_MASTER => "CALG_SSL3_MASTER",
        CALG_SSL3_SHAMD5 => "CALG_SSL3_SHAMD5",
        CALG_TEK => "CALG_TEK",
        CALG_TLS1_MASTER => "CALG_TLS1_MASTER",
        CALG_TLS1PRF => "CALG_TLS1PRF",
        _ => "Unknown Algorithm",
    }
}

/// Fixed key length in bytes. Zero denotes a variable length or an unknown algorithm.
pub fn get_crypto_key_len(value: u32) -> usize {
    match value {
        CALG_DES | CALG_DESX | CALG_RC2 => 8,
        CALG_3DES_112 => 14,
        CALG_AES_128 => 16,
        CALG_3DES | CALG_AES_192 => 24,
        CALG_AES_256 => 32,
        _ => 0,
    }
}

pub const CRYPTPROTECT_UI_FORBIDDEN: u32 = 0x1;

pub const BCRYPT_BLOCK_PADDING: u32 = 0x1;
pub const BCRYPT_RC4_ALG_HANDLE: u64 = 0x71;
pub const BCRYPT_HMAC_SHA1_ALG_HANDLE: u64 = 0xa1;
pub const BCRYPT_HMAC_SHA256_ALG_HANDLE: u64 = 0xb1;
pub const BCRYPT_3DES_CBC_ALG_HANDLE: u64 = 0x141;
pub const BCRYPT_AES_CBC_ALG_HANDLE: u64 = 0x1a1;
pub const BCRYPT_MD5_ALG_HANDLE: u64 = 0x21;
pub const BCRYPT_SHA1_ALG_HANDLE: u64 = 0x31;
pub const BCRYPT_SHA256_ALG_HANDLE: u64 = 0x41;
pub const BCRYPT_ALG_HANDLE_HMAC_FLAG: u32 = 0x8;
pub const BCRYPT_RNG_ALG_HANDLE: u64 = 0x81;
pub const BCRYPT_RNG_USE_ENTROPY_IN_BUFFER: u32 = 0x1;
pub const BCRYPT_USE_SYSTEM_PREFERRED_RNG: u32 = 0x2;

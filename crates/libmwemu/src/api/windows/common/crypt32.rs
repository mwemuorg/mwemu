use crate::windows::constants::crypto::*;
use sha1::Sha1;
use sha2::{Digest, Sha256};

use crate::emu;

fn base64_decode(input: &[u8]) -> Option<Vec<u8>> {
    let filtered: Vec<u8> = input
        .iter()
        .copied()
        .filter(|&b| !b.is_ascii_whitespace())
        .collect();

    let decode_char = |c: u8| -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'a'..=b'z' => Some(c - b'a' + 26),
            b'0'..=b'9' => Some(c - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            b'=' => Some(0),
            _ => None,
        }
    };

    if !filtered.len().is_multiple_of(4) {
        return None;
    }

    let mut out = Vec::with_capacity(filtered.len() * 3 / 4);
    for chunk in filtered.chunks(4) {
        let a = decode_char(chunk[0])?;
        let b = decode_char(chunk[1])?;
        let c = decode_char(chunk[2])?;
        let d = decode_char(chunk[3])?;

        let triple = (a as u32) << 18 | (b as u32) << 12 | (c as u32) << 6 | d as u32;

        out.push((triple >> 16) as u8);
        if chunk[2] != b'=' {
            out.push((triple >> 8) as u8);
        }
        if chunk[3] != b'=' {
            out.push(triple as u8);
        }
    }
    Some(out)
}

fn strip_pem_headers(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect::<Vec<_>>()
        .join("")
}

fn hex_decode(input: &[u8]) -> Option<Vec<u8>> {
    let filtered: Vec<u8> = input
        .iter()
        .copied()
        .filter(|b| b.is_ascii_hexdigit())
        .collect();
    if !filtered.len().is_multiple_of(2) {
        return None;
    }
    let mut out = Vec::with_capacity(filtered.len() / 2);
    for pair in filtered.chunks(2) {
        let hi = hex_nibble(pair[0])?;
        let lo = hex_nibble(pair[1])?;
        out.push(hi << 4 | lo);
    }
    Some(out)
}

fn hex_nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

pub(crate) fn crypt_string_to_binary_a(
    emu: &mut emu::Emu,
    string_ptr: u64,
    num_chars: u64,
    flags: u64,
    out_ptr: u64,
    inout_sz_ptr: u64,
) -> u64 {
    let input_str = if num_chars > 0 {
        let bytes = emu.maps.read_bytes(string_ptr, num_chars as usize);
        String::from_utf8_lossy(&bytes).into_owned()
    } else {
        emu.maps.read_string(string_ptr)
    };

    let decoded = match flags {
        CRYPT_STRING_BASE64 | CRYPT_STRING_BASE64_ANY => base64_decode(input_str.as_bytes()),
        CRYPT_STRING_BASE64HEADER => {
            let stripped = strip_pem_headers(&input_str);
            base64_decode(stripped.as_bytes())
        }
        CRYPT_STRING_BINARY | CRYPT_STRING_ANY => Some(input_str.as_bytes().to_vec()),
        CRYPT_STRING_HEX | CRYPT_STRING_HEX_ANY | CRYPT_STRING_HEXRAW => {
            hex_decode(input_str.as_bytes())
        }
        _ => Some(input_str.as_bytes().to_vec()),
    };

    let dflags = match flags {
        CRYPT_STRING_BASE64HEADER => "BASE64HEADER",
        CRYPT_STRING_BASE64 => "BASE64",
        CRYPT_STRING_BINARY => "BINARY",
        CRYPT_STRING_HEX => "HEX",
        CRYPT_STRING_BASE64_ANY => "BASE64_ANY",
        CRYPT_STRING_ANY => "ANY",
        CRYPT_STRING_HEX_ANY => "HEX_ANY",
        CRYPT_STRING_HEXRAW => "HEXRAW",
        _ => "unknown",
    };

    match decoded {
        Some(data) => {
            if out_ptr != 0 && inout_sz_ptr != 0 {
                let buf_sz = emu.maps.read_dword(inout_sz_ptr).unwrap_or(0) as usize;
                let write_len = data.len().min(buf_sz);
                emu.maps.write_bytes(out_ptr, &data[..write_len]);
            }
            if inout_sz_ptr != 0 {
                emu.maps.write_dword(inout_sz_ptr, data.len() as u32);
            }
            log_red!(
                emu,
                "crypt32!CryptStringToBinaryA {} decoded {} -> {} bytes",
                dflags,
                input_str.len(),
                data.len()
            );
            1
        }
        None => {
            log_red!(emu, "crypt32!CryptStringToBinaryA {} decode failed", dflags);
            0
        }
    }
}

fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((input.len() + 2) / 3 * 4);
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[((triple >> 18) & 0x3F) as usize] as char);
        out.push(TABLE[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            out.push(TABLE[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(TABLE[(triple & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

fn hex_encode(input: &[u8]) -> String {
    let mut out = String::with_capacity(input.len() * 2);
    for b in input {
        out.push_str(&format!("{:02x}", b));
    }
    out
}

pub(crate) fn crypt_binary_to_string_a(
    emu: &mut emu::Emu,
    data_ptr: u64,
    data_len: u64,
    flags: u64,
    str_ptr: u64,
    str_len_ptr: u64,
) -> u64 {
    let data = emu.maps.read_bytes(data_ptr, data_len as usize);

    let encoded = match flags & 0x0000_000f {
        CRYPT_STRING_BASE64 | CRYPT_STRING_BASE64_ANY => base64_encode(&data),
        CRYPT_STRING_BASE64HEADER => {
            let b64 = base64_encode(&data);
            format!(
                "-----BEGIN CERTIFICATE-----\r\n{}\r\n-----END CERTIFICATE-----\r\n",
                b64
            )
        }
        CRYPT_STRING_BINARY | CRYPT_STRING_ANY => String::from_utf8_lossy(&data).into_owned(),
        CRYPT_STRING_HEX | CRYPT_STRING_HEX_ANY | CRYPT_STRING_HEXRAW => hex_encode(&data),
        _ => base64_encode(&data),
    };

    let needed = encoded.len() as u32 + 1;

    if str_ptr == 0 {
        if str_len_ptr != 0 {
            emu.maps.write_dword(str_len_ptr, needed);
        }
        log_red!(
            emu,
            "crypt32!CryptBinaryToStringA size query -> {} chars",
            needed
        );
        return 1;
    }

    let buf_sz = if str_len_ptr != 0 {
        emu.maps.read_dword(str_len_ptr).unwrap_or(0) as usize
    } else {
        needed as usize
    };

    let write_len = encoded.len().min(buf_sz.saturating_sub(1));
    emu.maps
        .write_bytes(str_ptr, &encoded.as_bytes()[..write_len]);
    emu.maps.write_byte(str_ptr + write_len as u64, 0);
    if str_len_ptr != 0 {
        emu.maps.write_dword(str_len_ptr, needed);
    }

    log_red!(
        emu,
        "crypt32!CryptBinaryToStringA {} bytes -> {} chars",
        data_len,
        encoded.len()
    );
    1
}

pub(crate) fn crypt_binary_to_string_w(
    emu: &mut emu::Emu,
    data_ptr: u64,
    data_len: u64,
    flags: u64,
    str_ptr: u64,
    str_len_ptr: u64,
) -> u64 {
    let data = emu.maps.read_bytes(data_ptr, data_len as usize);

    let encoded = match flags & 0x0000_000f {
        CRYPT_STRING_BASE64 | CRYPT_STRING_BASE64_ANY => base64_encode(&data),
        CRYPT_STRING_BASE64HEADER => {
            let b64 = base64_encode(&data);
            format!(
                "-----BEGIN CERTIFICATE-----\r\n{}\r\n-----END CERTIFICATE-----\r\n",
                b64
            )
        }
        CRYPT_STRING_BINARY | CRYPT_STRING_ANY => String::from_utf8_lossy(&data).into_owned(),
        CRYPT_STRING_HEX | CRYPT_STRING_HEX_ANY | CRYPT_STRING_HEXRAW => hex_encode(&data),
        _ => base64_encode(&data),
    };

    let needed = encoded.len() as u32 + 1;

    if str_ptr == 0 {
        if str_len_ptr != 0 {
            emu.maps.write_dword(str_len_ptr, needed);
        }
        log_red!(
            emu,
            "crypt32!CryptBinaryToStringW size query -> {} chars",
            needed
        );
        return 1;
    }

    let buf_sz = if str_len_ptr != 0 {
        emu.maps.read_dword(str_len_ptr).unwrap_or(0) as usize
    } else {
        needed as usize
    };

    let write_len = encoded.len().min(buf_sz.saturating_sub(1));
    emu.maps.write_wide_string(str_ptr, &encoded[..write_len]);
    if str_len_ptr != 0 {
        emu.maps.write_dword(str_len_ptr, needed);
    }

    log_red!(
        emu,
        "crypt32!CryptBinaryToStringW {} bytes -> {} chars",
        data_len,
        encoded.len()
    );
    1
}

pub(crate) fn crypt_string_to_binary_w(
    emu: &mut emu::Emu,
    string_ptr: u64,
    num_chars: u64,
    flags: u64,
    out_ptr: u64,
    inout_sz_ptr: u64,
) -> u64 {
    let input_str = if num_chars > 0 {
        let mut s = String::new();
        for i in 0..num_chars {
            let c = emu.maps.read_word(string_ptr + i * 2).unwrap_or(0);
            if c == 0 {
                break;
            }
            s.push(char::from_u32(c as u32).unwrap_or('\0'));
        }
        s
    } else {
        emu.maps.read_wide_string(string_ptr)
    };

    let decoded = match flags {
        CRYPT_STRING_BASE64 | CRYPT_STRING_BASE64_ANY => base64_decode(input_str.as_bytes()),
        CRYPT_STRING_BASE64HEADER => {
            let stripped = strip_pem_headers(&input_str);
            base64_decode(stripped.as_bytes())
        }
        CRYPT_STRING_BINARY | CRYPT_STRING_ANY => Some(input_str.as_bytes().to_vec()),
        CRYPT_STRING_HEX | CRYPT_STRING_HEX_ANY | CRYPT_STRING_HEXRAW => {
            hex_decode(input_str.as_bytes())
        }
        _ => Some(input_str.as_bytes().to_vec()),
    };

    let dflags = match flags {
        0x00 => "BASE64HEADER",
        0x01 => "BASE64",
        0x02 => "BINARY",
        0x04 => "HEX",
        0x06 => "BASE64_ANY",
        0x07 => "ANY",
        0x08 => "HEX_ANY",
        0x0c => "HEXRAW",
        _ => "unknown",
    };

    match decoded {
        Some(data) => {
            if out_ptr != 0 && inout_sz_ptr != 0 {
                let buf_sz = emu.maps.read_dword(inout_sz_ptr).unwrap_or(0) as usize;
                let write_len = data.len().min(buf_sz);
                emu.maps.write_bytes(out_ptr, &data[..write_len]);
            }
            if inout_sz_ptr != 0 {
                emu.maps.write_dword(inout_sz_ptr, data.len() as u32);
            }
            log_red!(
                emu,
                "crypt32!CryptStringToBinaryW {} decoded {} -> {} bytes",
                dflags,
                input_str.len(),
                data.len()
            );
            1
        }
        None => {
            log_red!(emu, "crypt32!CryptStringToBinaryW {} decode failed", dflags);
            0
        }
    }
}

pub(crate) fn crypt_hash_certificate(
    emu: &mut emu::Emu,
    alg_id: u32,
    cert_encoded_ptr: u64,
    cert_encoded_len: u64,
    hash_ptr: u64,
    hash_len_ptr: u64,
) -> u64 {
    let data = emu
        .maps
        .read_bytes(cert_encoded_ptr, cert_encoded_len as usize);

    let (hash_bytes, hash_size) = match alg_id {
        CALG_SHA_256 | 0 => {
            let mut hasher = Sha256::new();
            hasher.update(data);
            let result = hasher.finalize();
            (result.to_vec(), 32u32)
        }
        CALG_SHA1 => {
            let mut hasher = Sha1::new();
            hasher.update(data);
            let result = hasher.finalize();
            (result.to_vec(), 20u32)
        }
        CALG_MD5 => {
            let digest = md5::compute(data);
            (digest.to_vec(), 16u32)
        }
        _ => {
            log_red!(
                emu,
                "crypt32!CryptHashCertificate unknown alg 0x{:x}",
                alg_id
            );
            return 0;
        }
    };

    if hash_ptr == 0 {
        if hash_len_ptr != 0 {
            emu.maps.write_dword(hash_len_ptr, hash_size);
        }
        log_red!(
            emu,
            "crypt32!CryptHashCertificate alg=0x{:x} size query -> {}",
            alg_id,
            hash_size
        );
        return 1;
    }

    let buf_sz = if hash_len_ptr != 0 {
        emu.maps.read_dword(hash_len_ptr).unwrap_or(0) as usize
    } else {
        hash_size as usize
    };

    let write_len = hash_bytes.len().min(buf_sz);
    emu.maps.write_bytes(hash_ptr, &hash_bytes[..write_len]);
    if hash_len_ptr != 0 {
        emu.maps.write_dword(hash_len_ptr, hash_size);
    }

    log_red!(
        emu,
        "crypt32!CryptHashCertificate alg=0x{:x} data={} bytes -> hash={} bytes",
        alg_id,
        cert_encoded_len,
        hash_size
    );
    1
}

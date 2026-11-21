use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::read_guest_bytes;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::crypto::*;
use crate::windows::constants::errors::*;
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::fmt::Write;

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

fn guest_bytes(emu: &Emu, pointer: u64, length: usize) -> Result<Vec<u8>, u64> {
    if length as u64 > emu.maps.max_alloc_size {
        return Err(ERROR_NOT_ENOUGH_MEMORY);
    }
    if length == 0 {
        return Ok(Vec::new());
    }
    read_guest_bytes(emu, pointer, length).ok_or(ERROR_NOACCESS)
}

fn decoded_base64(input: &[u8]) -> Option<Vec<u8>> {
    let mut bits = 0u32;
    let mut count = 0;
    let mut seen_padding = false;
    let mut output = Vec::new();
    for &ch in input {
        if ch.is_ascii_whitespace() {
            continue;
        }
        if ch == b'=' {
            seen_padding = true;
            continue;
        }
        if seen_padding {
            return None;
        }
        let sextet = match ch {
            b'A'..=b'Z' => ch - b'A',
            b'a'..=b'z' => ch - b'a' + 26,
            b'0'..=b'9' => ch - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => return None,
        };
        bits = (bits << 6) | u32::from(sextet);
        count += 6;
        if count >= 8 {
            count -= 8;
            output.push((bits >> count) as u8);
        }
    }
    if count == 6 {
        output.push((bits << 2) as u8);
    }
    Some(output)
}

fn pem(text: &[u8]) -> Option<(usize, Vec<u8>)> {
    let text = std::str::from_utf8(text).ok()?;
    let begin = text.find("-----BEGIN ")?;
    let rest = &text[begin + "-----BEGIN ".len()..];
    let (label, rest) = rest.split_once("-----")?;
    let (body, _) = rest.split_once(&format!("-----END {label}-----"))?;
    Some((begin, decoded_base64(body.as_bytes())?))
}

fn decode(
    emu: &Emu,
    wide: bool,
    input: u64,
    count: u32,
    flags: u64,
) -> Result<(Vec<u8>, usize, u64), u64> {
    if input == 0 {
        return Err(ERROR_INVALID_PARAMETER);
    }
    let raw = if count == 0 {
        let text = strings::read_text_checked(emu, input, wide).ok_or(ERROR_NOACCESS)?;
        strings::encode_text(&text, wide)
    } else {
        let length = (count as usize)
            .checked_mul(if wide { 2 } else { 1 })
            .ok_or(ERROR_INVALID_PARAMETER)?;
        guest_bytes(emu, input, length)?
    };
    if flags == CRYPT_STRING_BINARY {
        return Ok((raw, 0, CRYPT_STRING_BINARY));
    }
    let text = if wide {
        raw.as_chunks::<2>()
            .0
            .iter()
            .copied()
            .map(|v| u8::try_from(u16::from_le_bytes(v)))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ERROR_INVALID_DATA)?
    } else {
        raw.clone()
    };
    if matches!(
        flags,
        CRYPT_STRING_BASE64HEADER | CRYPT_STRING_BASE64_ANY | CRYPT_STRING_ANY
    ) {
        if let Some((skip, decoded)) = pem(&text) {
            return Ok((decoded, skip, CRYPT_STRING_BASE64HEADER));
        }
        if flags == CRYPT_STRING_BASE64HEADER {
            return Err(ERROR_INVALID_DATA);
        }
    }
    if matches!(
        flags,
        CRYPT_STRING_BASE64 | CRYPT_STRING_BASE64_ANY | CRYPT_STRING_ANY
    ) {
        if let Some(decoded) = decoded_base64(&text) {
            return Ok((decoded, 0, CRYPT_STRING_BASE64));
        }
        if flags == CRYPT_STRING_ANY {
            return Ok((raw, 0, CRYPT_STRING_BINARY));
        }
        return Err(ERROR_INVALID_DATA);
    }
    if matches!(
        flags,
        CRYPT_STRING_HEX | CRYPT_STRING_HEX_ANY | CRYPT_STRING_HEXRAW
    ) {
        if text
            .iter()
            .any(|byte| !byte.is_ascii_whitespace() && !byte.is_ascii_hexdigit())
        {
            return Err(ERROR_INVALID_DATA);
        }
        return hex_decode(&text)
            .map(|bytes| {
                (
                    bytes,
                    0,
                    if flags == CRYPT_STRING_HEX_ANY {
                        CRYPT_STRING_HEX
                    } else {
                        flags
                    },
                )
            })
            .ok_or(ERROR_INVALID_DATA);
    }
    Err(ERROR_INVALID_PARAMETER)
}

fn string_to_binary(emu: &mut Emu, name: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let [input, count, flags, output, size, skipped, format] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "crypt32!{name} input={input:#x} characters={count} flags={flags:#x} output={output:#x}"
    );
    let result = (|| {
        if size == 0
            || !emu.maps.validate_write_range(size, 4)
            || [skipped, format]
                .iter()
                .any(|&p| p != 0 && !emu.maps.validate_write_range(p, 4))
        {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let (data, skipped_chars, actual) =
            decode(emu, wide, input, count as u32, u64::from(flags as u32))?;
        if output != 0 {
            let capacity = emu.maps.read_dword(size).ok_or(ERROR_NOACCESS)? as usize;
            if capacity < data.len() {
                return Err(ERROR_MORE_DATA);
            }
            if !data.is_empty() && !emu.maps.write_bytes(output, &data) {
                return Err(ERROR_NOACCESS);
            }
        }
        if output == 0 || actual != CRYPT_STRING_BINARY {
            emu.maps.write_dword(size, data.len() as u32);
        }
        if skipped != 0 {
            emu.maps.write_dword(skipped, skipped_chars as u32);
        }
        if format != 0 {
            emu.maps.write_dword(format, actual as u32);
        }
        Ok(())
    })();
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 7);
}

pub fn CryptStringToBinaryA(emu: &mut Emu) {
    string_to_binary(emu, "CryptStringToBinaryA", false);
}
pub fn CryptStringToBinaryW(emu: &mut Emu) {
    string_to_binary(emu, "CryptStringToBinaryW", true);
}

fn encode(data: &[u8], flags: u64) -> Result<String, u64> {
    let separator = if flags & CRYPT_STRING_NOCRLF != 0 {
        ""
    } else if flags & CRYPT_STRING_NOCR != 0 {
        "\n"
    } else {
        "\r\n"
    };
    let format = flags & !(CRYPT_STRING_NOCRLF | CRYPT_STRING_NOCR);
    let mut text = String::new();
    match format {
        CRYPT_STRING_BASE64HEADER
        | CRYPT_STRING_BASE64
        | CRYPT_STRING_BASE64_ANY
        | CRYPT_STRING_ANY => {
            if format == CRYPT_STRING_BASE64HEADER {
                text.push_str("-----BEGIN CERTIFICATE-----");
                text.push_str(separator);
            }
            let encoded = base64_encode(data);
            for line in encoded.as_bytes().chunks(64) {
                for &byte in line {
                    text.push(char::from(byte));
                }
                text.push_str(separator);
            }
            if format == CRYPT_STRING_BASE64HEADER {
                text.push_str("-----END CERTIFICATE-----");
                text.push_str(separator);
            }
        }
        CRYPT_STRING_HEX | CRYPT_STRING_HEX_ANY | CRYPT_STRING_HEXRAW => {
            for row in data.chunks(if format == CRYPT_STRING_HEXRAW {
                usize::MAX
            } else {
                16
            }) {
                for (index, byte) in row.iter().enumerate() {
                    if index != 0 && format != CRYPT_STRING_HEXRAW {
                        text.push(' ');
                        if index == 8 {
                            text.push(' ');
                        }
                    }
                    write!(text, "{byte:02x}").map_err(|_| ERROR_NOT_ENOUGH_MEMORY)?;
                }
                text.push_str(separator);
            }
        }
        _ => return Err(ERROR_INVALID_PARAMETER),
    }
    Ok(text)
}

fn binary_to_string(emu: &mut Emu, name: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let [input, count, flags, output, size] = api_try!(emu, call.args(emu));
    let flags = u64::from(flags as u32);
    log_red!(
        emu,
        "crypt32!{name} input={input:#x} bytes={count} flags={flags:#x} output={output:#x}"
    );
    let result = (|| {
        if input == 0 || size == 0 || !emu.maps.validate_write_range(size, 4) {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let data = guest_bytes(emu, input, count as u32 as usize)?;
        let raw = flags & !(CRYPT_STRING_NOCRLF | CRYPT_STRING_NOCR) == CRYPT_STRING_BINARY;
        let (payload, used, needed, width) = if raw {
            let len = data.len();
            (data, len, len, 1)
        } else {
            let mut encoded = encode(&data, flags)?;
            let used = encoded.len();
            encoded.push('\0');
            (
                strings::encode_text(&encoded, wide),
                used,
                used + 1,
                if wide { 2 } else { 1 },
            )
        };
        if output == 0 {
            emu.maps.write_dword(size, needed as u32);
            return Ok(());
        }
        let capacity = emu.maps.read_dword(size).ok_or(ERROR_NOACCESS)? as usize;
        let length = capacity.min(needed) * width;
        if length != 0 && !emu.maps.write_bytes(output, &payload[..length]) {
            return Err(ERROR_NOACCESS);
        }
        if capacity < needed {
            return Err(ERROR_MORE_DATA);
        }
        emu.maps.write_dword(size, used as u32);
        Ok(())
    })();
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 5);
}

pub fn CryptBinaryToStringA(emu: &mut Emu) {
    binary_to_string(emu, "CryptBinaryToStringA", false);
}
pub fn CryptBinaryToStringW(emu: &mut Emu) {
    binary_to_string(emu, "CryptBinaryToStringW", true);
}

pub fn CryptHashCertificate(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [provider, algorithm, flags, input, length, output, size] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "crypt32!CryptHashCertificate provider={provider:#x} algorithm={algorithm:#x} flags={flags:#x} input={input:#x} bytes={length} output={output:#x}"
    );
    let result = (|| {
        if flags as u32 != 0 || size == 0 || !emu.maps.validate_write_range(size, 4) {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let bytes = guest_bytes(emu, input, length as u32 as usize)?;
        let digest = match algorithm as u32 {
            CALG_SHA_256 => Sha256::digest(&bytes).to_vec(),
            CALG_SHA1 | 0 => Sha1::digest(&bytes).to_vec(),
            CALG_MD5 => md5::compute(&bytes).to_vec(),
            _ => return Err(NTE_BAD_ALGID),
        };
        if output != 0 {
            let capacity = emu.maps.read_dword(size).ok_or(ERROR_NOACCESS)? as usize;
            if capacity < digest.len() {
                emu.maps.write_dword(size, digest.len() as u32);
                return Err(ERROR_MORE_DATA);
            }
            if !emu.maps.write_bytes(output, &digest) {
                return Err(ERROR_NOACCESS);
            }
        }
        emu.maps.write_dword(size, digest.len() as u32);
        Ok(())
    })();
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 7);
}

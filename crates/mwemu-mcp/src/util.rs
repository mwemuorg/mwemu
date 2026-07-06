//! Small helpers to extract and parse arguments out of a JSON `Value`.
//!
//! mwemu works with full 64-bit addresses, which do not survive a JSON number
//! (IEEE-754 doubles only hold 53 bits exactly). So every integer accepted by a
//! tool can be given either as a JSON number *or* as a string ("0x401000",
//! "4096"). Hex is recognised by the `0x` prefix, otherwise decimal is tried
//! first and hex as a fallback.

use serde_json::Value;

/// Parse a u64 out of a JSON value (number or "0x.."/decimal string).
pub fn parse_u64(v: &Value) -> Result<u64, String> {
    if let Some(n) = v.as_u64() {
        return Ok(n);
    }
    if let Some(n) = v.as_i64() {
        return Ok(n as u64);
    }
    if let Some(s) = v.as_str() {
        return parse_u64_str(s);
    }
    Err(format!("expected an integer or hex string, got {v}"))
}

pub fn parse_u64_str(s: &str) -> Result<u64, String> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        return u64::from_str_radix(hex, 16).map_err(|e| format!("invalid hex '{s}': {e}"));
    }
    if let Ok(n) = s.parse::<u64>() {
        return Ok(n);
    }
    // last resort: maybe it's bare hex without prefix
    u64::from_str_radix(s, 16).map_err(|_| format!("invalid integer '{s}'"))
}

/// Required integer field.
pub fn req_u64(a: &Value, key: &str) -> Result<u64, String> {
    match a.get(key) {
        Some(v) if !v.is_null() => parse_u64(v).map_err(|e| format!("'{key}': {e}")),
        _ => Err(format!("missing required field '{key}'")),
    }
}

/// Optional integer field (absent or null -> None).
pub fn opt_u64(a: &Value, key: &str) -> Result<Option<u64>, String> {
    match a.get(key) {
        Some(v) if !v.is_null() => Ok(Some(parse_u64(v).map_err(|e| format!("'{key}': {e}"))?)),
        _ => Ok(None),
    }
}

/// Required string field.
pub fn req_str<'a>(a: &'a Value, key: &str) -> Result<&'a str, String> {
    a.get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("missing required string field '{key}'"))
}

/// Optional string field.
pub fn opt_str<'a>(a: &'a Value, key: &str) -> Option<&'a str> {
    a.get(key).and_then(|v| v.as_str())
}

/// Optional boolean field (default when absent).
pub fn opt_bool(a: &Value, key: &str, default: bool) -> bool {
    a.get(key).and_then(|v| v.as_bool()).unwrap_or(default)
}

/// Decode a blob of bytes from either a `hex` string field or a `bytes` array
/// of integers. `hex` accepts spaced ("41 42"), prefixed ("0x41 0x42") or
/// contiguous ("4142") forms.
pub fn get_bytes(a: &Value) -> Result<Vec<u8>, String> {
    if let Some(s) = opt_str(a, "hex") {
        return hex_to_bytes(s);
    }
    if let Some(arr) = a.get("bytes").and_then(|v| v.as_array()) {
        let mut out = Vec::with_capacity(arr.len());
        for (i, e) in arr.iter().enumerate() {
            let n = e
                .as_u64()
                .ok_or_else(|| format!("bytes[{i}] is not a byte"))?;
            if n > 0xff {
                return Err(format!("bytes[{i}]={n} out of byte range"));
            }
            out.push(n as u8);
        }
        return Ok(out);
    }
    Err("provide a 'hex' string or a 'bytes' array".to_string())
}

/// Parse a hex string in spaced / prefixed / contiguous form into raw bytes.
pub fn hex_to_bytes(s: &str) -> Result<Vec<u8>, String> {
    let mut joined = String::new();
    for tok in s.replace(',', " ").split_whitespace() {
        let tok = tok.strip_prefix("0x").or_else(|| tok.strip_prefix("0X")).unwrap_or(tok);
        joined.push_str(tok);
    }
    if joined.len() % 2 != 0 {
        return Err(format!("hex string has an odd number of nibbles: '{s}'"));
    }
    let mut out = Vec::with_capacity(joined.len() / 2);
    let bytes = joined.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let hi = (bytes[i] as char)
            .to_digit(16)
            .ok_or_else(|| format!("invalid hex char '{}'", bytes[i] as char))?;
        let lo = (bytes[i + 1] as char)
            .to_digit(16)
            .ok_or_else(|| format!("invalid hex char '{}'", bytes[i + 1] as char))?;
        out.push((hi * 16 + lo) as u8);
        i += 2;
    }
    Ok(out)
}

/// Render bytes as a contiguous lowercase hex string.
pub fn bytes_to_hex(b: &[u8]) -> String {
    let mut s = String::with_capacity(b.len() * 2);
    for x in b {
        s.push_str(&format!("{x:02x}"));
    }
    s
}

/// Render bytes as a space-separated hex string (the form mwemu's search wants).
pub fn bytes_to_spaced_hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect::<Vec<_>>().join(" ")
}

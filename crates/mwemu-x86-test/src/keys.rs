//! Classification of x86Tester state keys and little-endian value decoding.
//!
//! The corpus canonicalizes every scalar register to its 64-bit name (`rax`,
//! `r12`, ...), flags to `flags`/`rflags`/`eflags`, vectors to `xmm<n>`/`ymm<n>`,
//! and memory cells to `mem[0x...]`. Anything else (x87 `st<n>`, `x87status`,
//! `mm<n>`, `zmm<n>`, control/debug registers) is not modelled by this harness
//! yet and its row is skipped as unsupported rather than risking a panic inside
//! libmwemu's `set_by_name`/`get_by_name` (which `unreachable!` on unknown
//! names).

/// The only scalar register names libmwemu is guaranteed to accept and that the
/// corpus actually emits. Kept deliberately narrow to avoid feeding an unknown
/// name into `set_by_name`/`get_by_name`.
pub const SCALAR_REGISTERS: [&str; 17] = [
    "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rsp", "rbp", "r8", "r9", "r10", "r11", "r12", "r13",
    "r14", "r15", "rip",
];

#[derive(Debug, Clone)]
pub enum Key {
    Scalar(String),
    Flags,
    Xmm(String),
    Ymm(String),
    Mem(u64),
    /// A key that is syntactically understood but not modelled yet (e.g. `st0`,
    /// `zmm3`); the row that references it is reported as a skip.
    Unsupported(String),
}

fn is_indexed(name: &str, prefix: &str, max: u64) -> bool {
    match name.strip_prefix(prefix) {
        Some(rest) if !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_digit()) => {
            rest.parse::<u64>().map(|n| n < max).unwrap_or(false)
        }
        _ => false,
    }
}

pub fn classify(raw: &str) -> Key {
    let key = raw.trim().to_ascii_lowercase();

    if key == "flags" || key == "rflags" || key == "eflags" {
        return Key::Flags;
    }
    if let Some(addr) = key.strip_prefix("mem[").and_then(|s| s.strip_suffix(']')) {
        return match parse_u64(addr) {
            Some(a) => Key::Mem(a),
            None => Key::Unsupported(raw.to_string()),
        };
    }
    if SCALAR_REGISTERS.contains(&key.as_str()) {
        return Key::Scalar(key);
    }
    if is_indexed(&key, "xmm", 16) {
        return Key::Xmm(key);
    }
    if is_indexed(&key, "ymm", 16) {
        return Key::Ymm(key);
    }
    Key::Unsupported(raw.to_string())
}

pub fn parse_u64(text: &str) -> Option<u64> {
    let t = text.trim();
    if let Some(hex) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        u64::from_str_radix(hex, 16).ok()
    } else {
        // Corpus addresses are hex without a prefix in some fields; try both.
        t.parse::<u64>()
            .ok()
            .or_else(|| u64::from_str_radix(t, 16).ok())
    }
}

pub fn le_u64(bytes: &[u8]) -> u64 {
    let mut value = 0u64;
    for (i, &b) in bytes.iter().take(8).enumerate() {
        value |= (b as u64) << (i * 8);
    }
    value
}

pub fn le_u128(bytes: &[u8]) -> u128 {
    let mut value = 0u128;
    for (i, &b) in bytes.iter().take(16).enumerate() {
        value |= (b as u128) << (i * 8);
    }
    value
}

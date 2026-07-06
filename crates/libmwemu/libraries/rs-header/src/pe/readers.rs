#[inline]
pub fn read_u8(raw: &[u8], off: usize) -> u8 {
    raw[off]
}

#[inline]
pub fn read_u16_le(raw: &[u8], off: usize) -> u16 {
    ((raw[off + 1] as u16) << 8) | (raw[off] as u16)
}

#[inline]
pub fn read_u32_le(raw: &[u8], off: usize) -> u32 {
    ((raw[off + 3] as u32) << 24)
        | ((raw[off + 2] as u32) << 16)
        | ((raw[off + 1] as u32) << 8)
        | (raw[off] as u32)
}

#[inline]
pub fn read_u64_le(raw: &[u8], off: usize) -> u64 {
    ((raw[off + 7] as u64) << 56)
        | ((raw[off + 6] as u64) << 48)
        | ((raw[off + 5] as u64) << 40)
        | ((raw[off + 4] as u64) << 32)
        | ((raw[off + 3] as u64) << 24)
        | ((raw[off + 2] as u64) << 16)
        | ((raw[off + 1] as u64) << 8)
        | (raw[off] as u64)
}

#[inline]
pub fn write_u32_le(raw: &mut [u8], off: usize, val: u32) {
    raw[off] = (val & 0x000000ff) as u8;
    raw[off + 1] = ((val & 0x0000ff00) >> 8) as u8;
    raw[off + 2] = ((val & 0x00ff0000) >> 16) as u8;
    raw[off + 3] = ((val & 0xff000000) >> 24) as u8;
}

#[inline]
pub fn write_u64_le(raw: &mut [u8], off: usize, val: u64) {
    raw[off] = (val & 0x00000000_000000ff) as u8;
    raw[off + 1] = ((val & 0x00000000_0000ff00) >> 8) as u8;
    raw[off + 2] = ((val & 0x00000000_00ff0000) >> 16) as u8;
    raw[off + 3] = ((val & 0x00000000_ff000000) >> 24) as u8;
    raw[off + 4] = ((val & 0x000000ff_00000000) >> 32) as u8;
    raw[off + 5] = ((val & 0x0000ff00_00000000) >> 40) as u8;
    raw[off + 6] = ((val & 0x00ff0000_00000000) >> 48) as u8;
    raw[off + 7] = ((val & 0xff000000_00000000) >> 56) as u8;
}

#[inline]
pub fn read_c_string(raw: &[u8], off: usize) -> String {
    if off >= raw.len() {
        return String::new();
    }

    let end = raw[off..]
        .iter()
        .position(|&b| b == 0)
        .map(|pos| off + pos)
        .unwrap_or(raw.len());

    match std::str::from_utf8(&raw[off..end]) {
        Ok(s) => s.to_string(),
        Err(_) => String::new(),
    }
}

#[inline]
pub fn read_c_string_with_max(raw: &[u8], off: usize, max_len: usize) -> String {
    if off >= raw.len() {
        return String::new();
    }

    let end_limit = (off + max_len).min(raw.len());
    let Some(pos) = raw[off..end_limit].iter().position(|&b| b == 0) else {
        return String::new();
    };

    let end = off + pos;
    match std::str::from_utf8(&raw[off..end]) {
        Ok(s) => s.to_string(),
        Err(_) => String::new(),
    }
}

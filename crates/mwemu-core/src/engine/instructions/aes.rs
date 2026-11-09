// Shared AES-NI primitives (not an instruction handler). The S-boxes are derived
// once from the GF(2^8) multiplicative inverse and the AES affine transform.

use lazy_static::lazy_static;

pub fn gmul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0u8;
    for _ in 0..8 {
        if b & 1 != 0 {
            p ^= a;
        }
        let hi = a & 0x80;
        a <<= 1;
        if hi != 0 {
            a ^= 0x1b;
        }
        b >>= 1;
    }
    p
}

pub fn gf_inv(a: u8) -> u8 {
    if a == 0 {
        return 0;
    }
    // a^254 == a^-1 in GF(2^8).
    let mut r = 1u8;
    for _ in 0..254 {
        r = gmul(r, a);
    }
    r
}

fn affine(x: u8) -> u8 {
    x ^ x.rotate_left(1) ^ x.rotate_left(2) ^ x.rotate_left(3) ^ x.rotate_left(4) ^ 0x63
}

lazy_static! {
    pub static ref SBOX: [u8; 256] = {
        let mut s = [0u8; 256];
        for (i, v) in s.iter_mut().enumerate() {
            *v = affine(gf_inv(i as u8));
        }
        s
    };
    pub static ref INV_SBOX: [u8; 256] = {
        let mut s = [0u8; 256];
        for i in 0..256 {
            s[SBOX[i] as usize] = i as u8;
        }
        s
    };
}

pub fn to_bytes(v: u128) -> [u8; 16] {
    v.to_le_bytes()
}
pub fn from_bytes(b: [u8; 16]) -> u128 {
    u128::from_le_bytes(b)
}

// State byte layout: s[col*4 + row].
pub fn sub_bytes(s: &mut [u8; 16]) {
    for x in s.iter_mut() {
        *x = SBOX[*x as usize];
    }
}
pub fn inv_sub_bytes(s: &mut [u8; 16]) {
    for x in s.iter_mut() {
        *x = INV_SBOX[*x as usize];
    }
}

pub fn shift_rows(s: &[u8; 16]) -> [u8; 16] {
    let mut o = [0u8; 16];
    for r in 0..4 {
        for c in 0..4 {
            o[c * 4 + r] = s[((c + r) % 4) * 4 + r];
        }
    }
    o
}
pub fn inv_shift_rows(s: &[u8; 16]) -> [u8; 16] {
    let mut o = [0u8; 16];
    for r in 0..4 {
        for c in 0..4 {
            o[c * 4 + r] = s[((c + 4 - r) % 4) * 4 + r];
        }
    }
    o
}

pub fn mix_columns(s: &[u8; 16]) -> [u8; 16] {
    let mut o = [0u8; 16];
    for c in 0..4 {
        let (a0, a1, a2, a3) = (s[c * 4], s[c * 4 + 1], s[c * 4 + 2], s[c * 4 + 3]);
        o[c * 4] = gmul(a0, 2) ^ gmul(a1, 3) ^ a2 ^ a3;
        o[c * 4 + 1] = a0 ^ gmul(a1, 2) ^ gmul(a2, 3) ^ a3;
        o[c * 4 + 2] = a0 ^ a1 ^ gmul(a2, 2) ^ gmul(a3, 3);
        o[c * 4 + 3] = gmul(a0, 3) ^ a1 ^ a2 ^ gmul(a3, 2);
    }
    o
}
pub fn inv_mix_columns(s: &[u8; 16]) -> [u8; 16] {
    let mut o = [0u8; 16];
    for c in 0..4 {
        let (a0, a1, a2, a3) = (s[c * 4], s[c * 4 + 1], s[c * 4 + 2], s[c * 4 + 3]);
        o[c * 4] = gmul(a0, 14) ^ gmul(a1, 11) ^ gmul(a2, 13) ^ gmul(a3, 9);
        o[c * 4 + 1] = gmul(a0, 9) ^ gmul(a1, 14) ^ gmul(a2, 11) ^ gmul(a3, 13);
        o[c * 4 + 2] = gmul(a0, 13) ^ gmul(a1, 9) ^ gmul(a2, 14) ^ gmul(a3, 11);
        o[c * 4 + 3] = gmul(a0, 11) ^ gmul(a1, 13) ^ gmul(a2, 9) ^ gmul(a3, 14);
    }
    o
}

pub fn sub_word(w: u32) -> u32 {
    let b = w.to_le_bytes();
    u32::from_le_bytes([
        SBOX[b[0] as usize],
        SBOX[b[1] as usize],
        SBOX[b[2] as usize],
        SBOX[b[3] as usize],
    ])
}

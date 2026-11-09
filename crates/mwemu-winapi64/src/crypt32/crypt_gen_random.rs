use crate::emu;

pub fn CryptGenRandom(emu: &mut emu::Emu) {
    let h_prov = emu.regs().rcx;
    let len = emu.regs().rdx as u32;
    let buffer = emu.regs().r8;
    log_red!(
        emu,
        "crypt32!CryptGenRandom prov: 0x{:x} len: {} buf: 0x{:x}",
        h_prov,
        len,
        buffer
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    // Fill buffer with deterministic pseudo-random bytes based on emu.pos
    if buffer != 0 {
        for i in 0..len {
            let byte = ((emu.pos as u32)
                .wrapping_mul(1103515245)
                .wrapping_add(i * 12345)) as u8;
            emu.maps.write_byte(buffer + i as u64, byte);
        }
    }
    emu.regs_mut().rax = 1; // TRUE
}

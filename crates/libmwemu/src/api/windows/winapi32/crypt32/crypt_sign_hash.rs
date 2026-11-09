use crate::emu;

pub fn CryptSignHash(emu: &mut emu::Emu) {
    let h_hash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let key_spec = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _desc_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let sig_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let sig_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "crypt32!CryptSignHash hash: 0x{:x} spec: {}",
        h_hash,
        key_spec
    );
    if sig_ptr == 0 {
        if sig_len_ptr != 0 {
            emu.maps.write_dword(sig_len_ptr, 128);
        }
    } else {
        for i in 0..128u64 {
            emu.maps.write_byte(sig_ptr + i, 0xAA);
        }
        if sig_len_ptr != 0 {
            emu.maps.write_dword(sig_len_ptr, 128);
        }
    }
    emu.regs_mut().rax = 1; // TRUE
}

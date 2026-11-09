use crate::emu;

pub fn CryptGenRandom(emu: &mut emu::Emu) {
    let h_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let len = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let buffer = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "crypt32!CryptGenRandom prov: 0x{:x} len: {} buf: 0x{:x}",
        h_prov,
        len,
        buffer
    );
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
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

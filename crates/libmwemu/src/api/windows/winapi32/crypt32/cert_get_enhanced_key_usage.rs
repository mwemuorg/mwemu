use crate::emu;

pub fn CertGetEnhancedKeyUsage(emu: &mut emu::Emu) {
    let p_cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p_usage = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let pcb_usage = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CertGetEnhancedKeyUsage 0x{:x}", p_cert_ctx);
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

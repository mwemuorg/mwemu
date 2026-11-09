use crate::emu;

pub fn GetNameInfoW(emu: &mut emu::Emu) {
    let sa_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let salen = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let host_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let hostlen = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "ws2_32!GetNameInfoW sa:0x{:x} host:0x{:x} hostlen:{}",
        sa_ptr,
        host_ptr,
        hostlen
    );
    if host_ptr != 0 && hostlen > 10 {
        emu.maps.write_wide_string(host_ptr, "localhost");
    }
    emu.regs_mut().rax = 0;
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}

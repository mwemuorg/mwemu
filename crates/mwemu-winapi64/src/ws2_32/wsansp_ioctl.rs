use crate::emu;

pub fn WSANSPIoctl(emu: &mut emu::Emu) {
    let lookup_handle = emu.regs().rcx;
    let control_code = emu.regs().rdx;
    log_red!(
        emu,
        "ws2_32!WSANSPIoctl handle:0x{:x} code:0x{:x}",
        lookup_handle,
        control_code
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

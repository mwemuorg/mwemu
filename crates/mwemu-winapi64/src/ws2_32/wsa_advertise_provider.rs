use crate::emu;

pub fn WSAAdvertiseProvider(emu: &mut emu::Emu) {
    let provider_id_ptr = emu.regs().rcx;
    let routine_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "ws2_32!WSAAdvertiseProvider id:0x{:x} routine:0x{:x}",
        provider_id_ptr,
        routine_ptr
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

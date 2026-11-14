use crate::emu;

pub fn WSAUnadvertiseProvider(emu: &mut emu::Emu) {
    let provider_id_ptr = emu.regs().rcx;
    log_red!(
        emu,
        "ws2_32!WSAUnadvertiseProvider id:0x{:x}",
        provider_id_ptr
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

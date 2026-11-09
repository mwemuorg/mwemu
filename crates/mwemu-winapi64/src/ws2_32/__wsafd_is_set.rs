use crate::emu;

pub fn __WSAFDIsSet(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let fd_set_ptr = emu.regs().rdx;
    log_red!(emu, "ws2_32!__WSAFDIsSet sock:0x{:x}", sock);
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

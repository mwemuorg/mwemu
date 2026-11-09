use crate::emu;

pub fn __WSAFDIsSet(emu: &mut emu::Emu) {
    let fd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let set_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!__WSAFDIsSet fd:0x{:x} set:0x{:x}", fd, set_ptr);
    let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}

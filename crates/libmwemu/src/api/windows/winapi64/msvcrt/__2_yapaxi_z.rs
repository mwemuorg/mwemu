use super::allocate_memory;
use crate::emu;

pub fn __2_YAPAXI_Z(emu: &mut emu::Emu) {
    let size = emu.regs().rcx;
    let base = allocate_memory(emu, size).unwrap_or(0);
    log_red!(emu, "msvcrt!operator_new size:{} => 0x{:x}", size, base);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = base;
}

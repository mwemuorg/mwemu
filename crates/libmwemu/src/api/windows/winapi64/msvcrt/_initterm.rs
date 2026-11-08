use crate::emu;

pub fn _initterm(emu: &mut emu::Emu) {
    let start_ptr = emu.regs().rcx;
    let end_ptr = emu.regs().rdx;
    log_red!(emu, "msvcrt!_initterm 0x{:x} - 0x{:x}", start_ptr, end_ptr);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    let mut ptr = start_ptr;
    while ptr < end_ptr {
        let func = emu.maps.read_qword(ptr).unwrap_or(0);
        if func != 0 {
            log_red!(emu, "  _initterm: init func 0x{:x}", func);
        }
        ptr += 8;
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

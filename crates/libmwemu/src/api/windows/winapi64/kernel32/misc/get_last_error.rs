use crate::emu;

pub fn GetLastError(emu: &mut emu::Emu) {
    let err = emu.get_last_error();
    log_red!(emu, "kernel32!GetLastError -> {}", err);
    emu.regs_mut().rax = err;
}

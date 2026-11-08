use crate::emu;

pub fn Process32NextW_api(emu: &mut emu::Emu) {
    let snap = emu.regs().rcx;
    let entry = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!Process32NextW snap:0x{:x} entry:0x{:x} => no more",
        snap,
        entry
    );
    emu.set_last_error(18); // ERROR_NO_MORE_FILES
    emu.regs_mut().rax = 0;
}

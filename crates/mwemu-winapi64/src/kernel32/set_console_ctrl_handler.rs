use crate::emu;

pub fn SetConsoleCtrlHandler(emu: &mut emu::Emu) {
    let handler_routine = emu.regs().rcx;
    let add = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!SetConsoleCtrlHandler handler:0x{:x} add:{}",
        handler_routine,
        add
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}

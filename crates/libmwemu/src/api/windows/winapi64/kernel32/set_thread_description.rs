use crate::emu;

pub fn SetThreadDescription(emu: &mut emu::Emu) {
    let h_thread = emu.regs().rcx;
    let description_ptr = emu.regs().rdx;
    let desc = emu.maps.read_wide_string(description_ptr);
    log_red!(
        emu,
        "kernel32!SetThreadDescription hThread:0x{:x} desc:'{}'",
        h_thread,
        desc
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

use crate::emu;

pub fn DelayLoadFailureHook(emu: &mut emu::Emu) {
    let dll_name = emu.regs().rcx;
    let p_delay_info = emu.regs().rdx;

    let name = emu.maps.read_string(dll_name);
    log_red!(
        emu,
        "kernel32!DelayLoadFailureHook dll:'{}' pDelayInfo:0x{:x}",
        name,
        p_delay_info
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

use crate::emu;

pub fn GetThreadPriorityBoost(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let disable_ptr = emu.regs().rdx;
    if disable_ptr != 0 {
        emu.maps.write_dword(disable_ptr, 0); // boost not disabled
    }
    log_red!(
        emu,
        "kernelbase!GetThreadPriorityBoost h:0x{:x} => FALSE",
        handle
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

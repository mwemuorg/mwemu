use crate::emu;

pub fn CoVrfGetThreadState(emu: &mut emu::Emu) {
    let state_ptr = emu.regs().rcx;
    log_red!(emu, "ole32!CoVrfGetThreadState state_ptr:0x{:x}", state_ptr);
    if state_ptr != 0 {
        emu.maps.write_dword(state_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

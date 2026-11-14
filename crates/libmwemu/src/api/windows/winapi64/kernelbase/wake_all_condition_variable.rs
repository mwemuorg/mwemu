use crate::emu;
use crate::winapi::helper;

pub fn WakeAllConditionVariable(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "kernelbase!WakeAllConditionVariable 0x{:x}", arg1);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

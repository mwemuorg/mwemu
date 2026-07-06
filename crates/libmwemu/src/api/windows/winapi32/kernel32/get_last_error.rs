use crate::emu;
use crate::winapi::winapi32::kernel32::LAST_ERROR;

pub fn GetLastError(emu: &mut emu::Emu) {
    let err = LAST_ERROR.lock().unwrap();
    emu.regs_mut().rax = *err as u64;
    log_red!(emu, "kernel32!GetLastError ={}", emu.regs().rax);
}

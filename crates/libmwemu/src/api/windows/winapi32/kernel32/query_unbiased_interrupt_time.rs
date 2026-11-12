use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn QueryUnbiasedInterruptTime(emu: &mut emu::Emu) {
    let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(
        emu,
        "kernel32!QueryUnbiasedInterruptTime h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    stdcall_ret(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

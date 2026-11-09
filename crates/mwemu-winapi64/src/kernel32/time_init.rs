
use crate::windows::constants::errors::*;
use crate::emu;
use crate::winapi::helper;

pub fn handler(emu: &mut emu::Emu) {
    let _InitOnce = emu.regs().rcx;
    let _Parameter = emu.regs().rdx;
    let _Context = emu.regs().r8;
    log_red!(emu, "kernel32!TimeInit");
    let handle = helper::handler_create("TimeInit");
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
    for _ in 0..3 {}
}

use crate::emu;
use crate::winapi::helper;

pub fn GetStdHandle(emu: &mut emu::Emu) {
    let std_handle = emu.regs().rcx;
    let name = match std_handle as i64 {
        -10 => "STD_INPUT",
        -11 => "STD_OUTPUT",
        -12 => "STD_ERROR",
        _ => "UNKNOWN",
    };
    let hndl = helper::handler_create(&format!("std_{}", name));
    log_red!(emu, "kernelbase!GetStdHandle {} => 0x{:x}", name, hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}

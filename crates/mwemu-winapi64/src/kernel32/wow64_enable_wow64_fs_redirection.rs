use crate::emu;
use crate::winapi::helper;
pub fn Wow64EnableWow64FsRedirection(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!Wow64EnableWow64FsRedirection param1:0x{:x}",
        param1
    );
    let handle = helper::handler_create("Wow64EnableWow64FsRedirection");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}

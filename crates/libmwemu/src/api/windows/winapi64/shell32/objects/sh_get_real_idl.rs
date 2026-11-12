use crate::emu;
use crate::winapi::helper;

pub fn SHGetRealIDL(emu: &mut emu::Emu) {
    let psf = emu.regs().rcx;
    let pidl_simple = emu.regs().rdx;
    let ppidl_real = emu.regs().r8;
    let hndl = helper::handler_create("pidl://real");
    if ppidl_real != 0 {
        emu.maps.write_qword(ppidl_real, hndl);
    }
    log_red!(emu, "shell32!SHGetRealIDL =0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

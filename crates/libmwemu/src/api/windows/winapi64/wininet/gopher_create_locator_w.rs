use crate::emu;
use crate::winapi::helper;

pub fn GopherCreateLocatorW(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "wininet!GopherCreateLocatorW arg1:0x{:x}", arg1);
    let handle = helper::handler_create("GopherCreateLocatorW");
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(2);
    emu.regs_mut().rax = 0;
}

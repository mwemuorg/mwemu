use crate::emu;

pub fn GopherCreateLocatorA(emu: &mut emu::Emu) {
    gopher_create_locator(emu, "GopherCreateLocatorA");
}

fn gopher_create_locator(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "wininet!{api} arg1:0x{:x}", arg1);
    emu.set_last_error(2);
    emu.regs_mut().rax = 0;
}

pub fn GopherCreateLocatorW(emu: &mut emu::Emu) {
    gopher_create_locator(emu, "GopherCreateLocatorW");
}

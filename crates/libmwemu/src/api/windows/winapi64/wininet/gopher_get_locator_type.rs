use crate::emu;

pub fn GopherGetLocatorTypeA(emu: &mut emu::Emu) {
    gopher_get_locator_type(emu, "GopherGetLocatorTypeA");
}

fn gopher_get_locator_type(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "wininet!{api} arg1:0x{:x}", arg1);
    emu.set_last_error(2);
    emu.regs_mut().rax = 0;
}

pub fn GopherGetLocatorTypeW(emu: &mut emu::Emu) {
    gopher_get_locator_type(emu, "GopherGetLocatorTypeW");
}

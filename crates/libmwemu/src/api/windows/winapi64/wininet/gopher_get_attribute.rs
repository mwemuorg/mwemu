use crate::emu;

pub fn GopherGetAttributeA(emu: &mut emu::Emu) {
    gopher_get_attribute(emu, "GopherGetAttributeA");
}

fn gopher_get_attribute(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "wininet!{api} arg1:0x{:x}", arg1);
    emu.set_last_error(2);
    emu.regs_mut().rax = 0;
}

pub fn GopherGetAttributeW(emu: &mut emu::Emu) {
    gopher_get_attribute(emu, "GopherGetAttributeW");
}

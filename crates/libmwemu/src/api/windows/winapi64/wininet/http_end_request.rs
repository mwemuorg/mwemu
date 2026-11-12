use crate::emu;
use crate::winapi::helper;

pub fn HttpEndRequestA(emu: &mut emu::Emu) {
    http_end_request(emu, "HttpEndRequestA");
}

fn http_end_request(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    log_red!(
        emu,
        "wininet!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    if arg1 != 0 && !helper::handler_exist(arg1) {
        emu.set_last_error(12019);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn HttpEndRequestW(emu: &mut emu::Emu) {
    http_end_request(emu, "HttpEndRequestW");
}

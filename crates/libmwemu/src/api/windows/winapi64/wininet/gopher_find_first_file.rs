use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn GopherFindFirstFileA(emu: &mut emu::Emu) {
    gopher_find_first_file(emu, "GopherFindFirstFileA");
}

fn gopher_find_first_file(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = WinApiAbi::arg(emu, 4);
    log_red!(
        emu,
        "wininet!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );

    emu.set_last_error(2);
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
}

pub fn GopherFindFirstFileW(emu: &mut emu::Emu) {
    gopher_find_first_file(emu, "GopherFindFirstFileW");
}

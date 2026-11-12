use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn FindFirstFileExA(emu: &mut emu::Emu) {
    find_first_file_ex(emu, "FindFirstFileExA");
}

fn find_first_file_ex(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = WinApiAbi::arg(emu, 4);
    let arg6 = WinApiAbi::arg(emu, 5);
    log_red!(
        emu,
        "kernelbase!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6
    );
    let hndl = helper::handler_create(&format!("findfile://{api}"));
    emu.regs_mut().rax = hndl;
}

pub fn FindFirstFileExW(emu: &mut emu::Emu) {
    find_first_file_ex(emu, "FindFirstFileExW");
}

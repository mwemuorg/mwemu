use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn ILCreateFromPath(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(
        emu,
        "shell32!ILCreateFromPath arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("ilcreatefrompath");
    emu.regs_mut().rax = handle;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}

pub fn ILCreateFromPathA(emu: &mut emu::Emu) {
    il_create_from_path(emu, "ILCreateFromPathA", false);
}

pub fn ILCreateFromPathW(emu: &mut emu::Emu) {
    il_create_from_path(emu, "ILCreateFromPathW", true);
}

fn il_create_from_path(emu: &mut emu::Emu, api: &str, wide: bool) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    if wide {
        let handle = helper::handler_create("ilcreatefrompathw");
        emu.regs_mut().rax = handle;
        stdcall_ret(emu, 1);
    } else {
        let handle = helper::handler_create("ilcreatefrompatha");
        emu.regs_mut().rax = handle;
        stdcall_ret(emu, 1);
    }
}

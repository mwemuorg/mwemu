use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn RemoveFontResourceExA(emu: &mut emu::Emu) {
    remove_font_resource_ex(emu, "RemoveFontResourceExA");
}

fn remove_font_resource_ex(emu: &mut emu::Emu, api: &str) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let arg3 = WinApiAbi::arg(emu, 2);
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "gdi32!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

pub fn RemoveFontResourceExW(emu: &mut emu::Emu) {
    remove_font_resource_ex(emu, "RemoveFontResourceExW");
}

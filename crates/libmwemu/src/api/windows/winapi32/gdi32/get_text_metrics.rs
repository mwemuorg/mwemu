use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetTextMetricsA(emu: &mut emu::Emu) {
    get_text_metrics(emu, "GetTextMetricsA");
}

fn get_text_metrics(emu: &mut emu::Emu, api: &str) {
    let hdc = arg(emu, 0);
    let tm = arg(emu, 1) as u64;
    if tm != 0 {
        emu.maps.write_dword(tm, 16); // tmHeight
        emu.maps.write_dword(tm + 4, 13); // tmAscent
        emu.maps.write_dword(tm + 8, 3); // tmDescent
        emu.maps.write_dword(tm + 12, 0); // tmInternalLeading
        emu.maps.write_dword(tm + 16, 0); // tmExternalLeading
        emu.maps.write_dword(tm + 20, 7); // tmAveCharWidth
        emu.maps.write_dword(tm + 24, 14); // tmMaxCharWidth
    }

    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    log_red!(emu, "gdi32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

pub fn GetTextMetricsW(emu: &mut emu::Emu) {
    get_text_metrics(emu, "GetTextMetricsW");
}

use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn GetFinalPathNameByHandleA(emu: &mut emu::Emu) {
    get_final_path_name_by_handle(emu, "GetFinalPathNameByHandleA", false);
}

fn get_final_path_name_by_handle(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let p2 = WinApiAbi::arg(emu, 2);
    let p3 = WinApiAbi::arg(emu, 3);
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!{api} '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub fn GetFinalPathNameByHandleW(emu: &mut emu::Emu) {
    get_final_path_name_by_handle(emu, "GetFinalPathNameByHandleW", true);
}

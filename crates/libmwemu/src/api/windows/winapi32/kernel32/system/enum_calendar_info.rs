use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn EnumCalendarInfoA(emu: &mut emu::Emu) {
    let _lpCalInfoEnumProc = WinApiAbi::arg(emu, 0) as u32;
    let _Locale = WinApiAbi::arg(emu, 1) as u32;
    let _Calendar = WinApiAbi::arg(emu, 2) as u32;
    let _CalType = WinApiAbi::arg(emu, 3) as u32;

    log_red!(emu, "kernel32!EnumCalendarInfoA");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub fn EnumCalendarInfoW(emu: &mut emu::Emu) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let p2 = WinApiAbi::arg(emu, 2);
    let p3 = WinApiAbi::arg(emu, 3);
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!EnumCalendarInfoW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

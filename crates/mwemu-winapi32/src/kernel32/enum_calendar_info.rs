use crate::emu;
use crate::windows::constants::errors::*;

pub fn EnumCalendarInfoA(emu: &mut emu::Emu) {
    let _lpCalInfoEnumProc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _Locale = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _Calendar = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _CalType = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    log_red!(emu, "kernel32!EnumCalendarInfoA");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub fn EnumCalendarInfoW(emu: &mut emu::Emu) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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

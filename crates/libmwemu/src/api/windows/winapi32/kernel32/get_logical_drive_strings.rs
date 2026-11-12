use crate::emu;
use crate::winapi::common::strings;

pub fn GetLogicalDriveStringsA(emu: &mut emu::Emu) {
    get_logical_drive_strings(emu, "GetLogicalDriveStringsA", false);
}

fn get_logical_drive_strings(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernel32!{api} '{}' p1:0x{:x}", s, p1);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 4;
}

pub fn GetLogicalDriveStringsW(emu: &mut emu::Emu) {
    get_logical_drive_strings(emu, "GetLogicalDriveStringsW", true);
}

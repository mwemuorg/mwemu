use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn FindResourceExA(emu: &mut emu::Emu) {
    find_resource_ex(emu, "FindResourceExA", false);
}

fn find_resource_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    let hndl = helper::handler_create("resource");
    log_red!(
        emu,
        "kernel32!{api} '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x} => 0x{:x}",
        s,
        p1,
        p2,
        p3,
        hndl
    );
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = hndl;
}

pub fn FindResourceExW(emu: &mut emu::Emu) {
    find_resource_ex(emu, "FindResourceExW", true);
}

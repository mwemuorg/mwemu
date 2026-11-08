use crate::emu;
use crate::winapi::helper;

pub fn FindResourceExA_api(emu: &mut emu::Emu) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    let hndl = helper::handler_create("resource");
    log_red!(
        emu,
        "kernel32!FindResourceExA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x} => 0x{:x}",
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

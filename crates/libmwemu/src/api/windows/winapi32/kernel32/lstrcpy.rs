use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn lstrcpy(emu: &mut emu::Emu) {
    let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = emu.maps.read_string(src);
    emu.maps.write_string(dst, &s);

    log_red!(emu, "kernel32!lstrcpy 0x{:x} `{}`", dst, s);

    emu.regs_mut().rax = dst;
    emu.stack_pop32(false);
    emu.stack_pop32(false);
}

pub fn lstrcpyA_api(emu: &mut emu::Emu) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernel32!lstrcpyA '{}' p1:0x{:x}", s, p1);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn lstrcpyW(emu: &mut emu::Emu) {
    let dst = arg(emu, 0) as u64;
    let src = arg(emu, 1) as u64;
    let s = emu.maps.read_wide_string(src);
    emu.maps.write_wide_string(dst, &s);
    log_red!(emu, "kernel32!lstrcpyW 0x{:x} 0x{:x} '{}'", dst, src, s);
    emu.regs_mut().rax = dst;
    stdcall_ret(emu, 2);
}

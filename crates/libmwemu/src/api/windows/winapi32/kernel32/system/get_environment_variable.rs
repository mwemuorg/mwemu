use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetEnvironmentVariableA(emu: &mut emu::Emu) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let p2 = WinApiAbi::arg(emu, 2);
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!GetEnvironmentVariableA '{}' p1:0x{:x} p2:0x{:x}",
        s,
        p1,
        p2
    );
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn GetEnvironmentVariableW(emu: &mut emu::Emu) {
    let name = emu.maps.read_wide_string(arg(emu, 0) as u64);
    log_red!(emu, "kernel32!GetEnvironmentVariableW `{}` (not set)", name);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

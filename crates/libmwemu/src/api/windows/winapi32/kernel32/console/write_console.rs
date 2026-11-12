use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn WriteConsoleA(emu: &mut emu::Emu) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let p2 = WinApiAbi::arg(emu, 2);
    let p3 = WinApiAbi::arg(emu, 3);
    let p4 = WinApiAbi::arg(emu, 4);
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!WriteConsoleA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn WriteConsoleW(emu: &mut emu::Emu) {
    let buf = arg(emu, 1) as u64;
    let count = arg(emu, 2) as usize;
    let written_ptr = arg(emu, 3) as u64;
    let text = emu.maps.read_wide_string_n(buf, count);
    log_red!(emu, "kernel32!WriteConsoleW `{}`", text);
    if written_ptr != 0 {
        emu.maps.write_dword(written_ptr, count as u32);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}

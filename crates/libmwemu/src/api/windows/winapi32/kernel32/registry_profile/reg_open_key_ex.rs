use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn RegOpenKeyExA(emu: &mut emu::Emu) {
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
        "kernel32!RegOpenKeyExA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}

pub fn RegOpenKeyExW(emu: &mut emu::Emu) {
    let hkey = WinApiAbi::arg(emu, 0) as u32;
    let subkey_ptr = WinApiAbi::arg(emu, 1);
    let options = WinApiAbi::arg(emu, 2) as u32;
    let sam = WinApiAbi::arg(emu, 3) as u32;
    let result = WinApiAbi::arg(emu, 4) as u32;

    let subkey = emu.maps.read_wide_string(subkey_ptr);
    log_red!(emu, "kernel32!RegOpenKeyExW {}", subkey);

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1;
}

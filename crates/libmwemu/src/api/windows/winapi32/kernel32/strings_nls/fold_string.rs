use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn FoldStringA(emu: &mut emu::Emu) {
    let _dwMapFlags = WinApiAbi::arg(emu, 0) as u32;
    let lpSrcStr = WinApiAbi::arg(emu, 1) as u32;
    let _cchSrc = WinApiAbi::arg(emu, 2) as u32;
    let lpDestStr = WinApiAbi::arg(emu, 3) as u32;
    let _cchDest = WinApiAbi::arg(emu, 4) as u32;

    let lpSrcStr = emu.maps.read_string(lpSrcStr as u64);
    let lpDestStr = emu.maps.read_string(lpDestStr as u64);

    log_red!(emu, "kernel32!FoldStringA {} {}", lpSrcStr, lpDestStr);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}

pub fn FoldStringW(emu: &mut emu::Emu) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let p2 = WinApiAbi::arg(emu, 2);
    let p3 = WinApiAbi::arg(emu, 3);
    let p4 = WinApiAbi::arg(emu, 4);
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!FoldStringW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}

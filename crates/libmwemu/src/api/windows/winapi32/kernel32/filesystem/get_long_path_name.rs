use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetLongPathNameA(emu: &mut emu::Emu) {
    let lpszShortPath = WinApiAbi::arg(emu, 0) as u32;
    let lpszLongPath = WinApiAbi::arg(emu, 1) as u32;
    let _cchBuffer = WinApiAbi::arg(emu, 2) as u32;

    let lpszShortPath = emu.maps.read_string(lpszShortPath as u64);
    let lpszLongPath = emu.maps.read_string(lpszLongPath as u64);

    log_red!(
        emu,
        "kernel32!GetLongPathNameA {} {}",
        lpszShortPath,
        lpszLongPath
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn GetLongPathNameW(emu: &mut emu::Emu) {
    let short_path_ptr = WinApiAbi::arg(emu, 0);
    let long_path_ptr = WinApiAbi::arg(emu, 1);
    let buff = WinApiAbi::arg(emu, 2);

    let short = emu.maps.read_wide_string(short_path_ptr);

    log_red!(
        emu,
        "kernel32!GetLongPathNameW  {} {:x}",
        short,
        long_path_ptr
    );

    if long_path_ptr > 0 {
        let mut base = String::from("\\.\\");
        base.push_str(&short);
        emu.maps.write_wide_string(long_path_ptr, &base);
    }

    emu.regs_mut().rax = short.len() as u64;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

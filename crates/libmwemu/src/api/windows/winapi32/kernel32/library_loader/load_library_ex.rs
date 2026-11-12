use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::kernel32::load_library;

pub fn LoadLibraryExA(emu: &mut emu::Emu) {
    let libname_ptr = WinApiAbi::arg(emu, 0);
    let libname = emu.maps.read_string(libname_ptr);

    log_red!(emu, "kernel32!LoadLibraryExA '{}'", libname);

    emu.regs_mut().rax = load_library(emu, &libname);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
}

pub fn LoadLibraryExW(emu: &mut emu::Emu) {
    let libname_ptr = WinApiAbi::arg(emu, 0);
    let hfile = WinApiAbi::arg(emu, 1);
    let flags = WinApiAbi::arg(emu, 2);

    let libname = emu.maps.read_wide_string(libname_ptr);

    log_red!(emu, "LoadLibraryExW '{}'", libname);

    emu.regs_mut().rax = load_library(emu, &libname);

    /*
    if emu.regs_mut().rax == 0 {
        emu.regs_mut().rax = 1;
    }*/

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
}

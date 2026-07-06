use crate::emu;
use crate::winapi::winapi32::kernel32::load_library;

pub fn LoadLibraryExW(emu: &mut emu::Emu) {
    let libname_ptr =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("kernel32!LoadLibraryExW: error reading libname ptr param") as u64;
    let hfile = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!LoadLibraryExW: error reading hFile") as u64;
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!LoadLibraryExW: error reading flags") as u64;

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

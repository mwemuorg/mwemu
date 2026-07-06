use crate::emu;
use crate::winapi::winapi32::kernel32::load_library;

pub fn LoadLibraryExA(emu: &mut emu::Emu) {
    let libname_ptr =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("kernel32_LoadLibraryExA: error reading libname ptr param") as u64;
    let libname = emu.maps.read_string(libname_ptr);

    log_red!(emu, "kernel32!LoadLibraryExA '{}'", libname);

    emu.regs_mut().rax = load_library(emu, &libname);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
}

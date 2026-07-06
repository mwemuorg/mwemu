use crate::emu;
use crate::winapi::winapi32::kernel32::load_library;

pub fn LoadLibraryW(emu: &mut emu::Emu) {
    let dllptr = match emu.maps.read_dword(emu.regs().get_esp()) {
        Some(v) => v as u64,
        None => panic!("bad LoadLibraryW parameter"),
    };
    let dll = emu.maps.read_wide_string(dllptr);
    log_red!(emu, "LoadLibraryW  '{}'", dll);

    //if dll == "ntdll.dll" {
    //  emu.regs_mut().rax = emu.maps.get_mem("ntdll").get_base();
    //}

    emu.regs_mut().rax = load_library(emu, &dll);

    emu.stack_pop32(false);
}

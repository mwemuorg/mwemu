use crate::api::windows::winapi32;
use crate::emu;
use crate::windows::constants;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "LdrLoadDll" => LdrLoadDll(emu),
        "LdrLoadDll_gul" => LdrLoadDll_gul(emu),
        _ => return false,
    }
    true
}

fn LdrLoadDll(emu: &mut emu::Emu) {
    let libname_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("ntdll!LdrLoadDll error reading libname param") as u64;
    let libaddr_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("ntdll!LdrLoadDll error reading libaddr param") as u64;

    let libname = emu.maps.read_wide_string(libname_ptr);
    log_red!(emu, "ntdll!LdrLoadDll   lib: {}", libname);

    let base = winapi32::kernel32::load_library(emu, &libname);
    if base != 0 {
        emu.maps.write_dword(libaddr_ptr, base as u32);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn LdrLoadDll_gul(emu: &mut emu::Emu) {
    LdrLoadDll(emu);
}

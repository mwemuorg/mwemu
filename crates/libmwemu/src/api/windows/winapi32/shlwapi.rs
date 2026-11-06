use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;
//use crate::winapi::helper;
//use crate::endpoint;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "PathFileExistsA" => PathFileExistsA(emu),
        "PathFileExistsW" => PathFileExistsW(emu),
        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}

fn PathFileExistsA(emu: &mut emu::Emu) {
    let ptr_path = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    let path = emu.maps.read_string(ptr_path);

    log_red!(emu, "shlwapi!PathFileExistsA path: {}", path);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1; // True
}

fn PathFileExistsW(emu: &mut emu::Emu) {
    let ptr_path = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    let path = emu.maps.read_wide_string(ptr_path);

    log_red!(emu, "shlwapi!PathFileExistsW path: {}", path);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1; // True
}

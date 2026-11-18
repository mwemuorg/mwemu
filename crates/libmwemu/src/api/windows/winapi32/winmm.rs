use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split('!').next_back().unwrap_or(&api);
    match api {
        "timeGetTime" => timeGetTime(emu),
        _ => return fallback::unimplemented(addr, api, emu),
    }
    String::new()
}

fn timeGetTime(emu: &mut emu::Emu) {
    log_red!(emu, "winmm!timeGetTime");
    emu.regs_mut().rax = emu.tick as u64;
}

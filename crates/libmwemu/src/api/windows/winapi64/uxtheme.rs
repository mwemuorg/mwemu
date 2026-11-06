use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "IsAppThemed" => IsAppThemed(emu),
        "IsThemeActive" => IsThemeActive(emu),
        "GetThemeAppProperties" => GetThemeAppProperties(emu),
        _ => return fallback::unimplemented(addr, api, emu),
    }
    String::new()
}

fn IsAppThemed(emu: &mut emu::Emu) {
    log_red!(emu, "** {} uxtheme!IsAppThemed", emu.pos);
    emu.regs_mut().rax = 1;
}

fn IsThemeActive(emu: &mut emu::Emu) {
    log_red!(emu, "** {} uxtheme!IsThemeActive", emu.pos);
    emu.regs_mut().rax = 1;
}

fn GetThemeAppProperties(emu: &mut emu::Emu) {
    log_red!(emu, "** {} uxtheme!GetThemeAppProperties", emu.pos);
    emu.regs_mut().rax = 1;
}

use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "InitCommonControls" => InitCommonControls(emu),
        "InitCommonControlsEx" => InitCommonControlsEx(emu),
        _ => return fallback::unimplemented(addr, api, emu),
    }
    String::new()
}

/*
void InitCommonControls();
*/
fn InitCommonControls(emu: &mut emu::Emu) {
    log_red!(emu, "comctl32!InitCommonControls");
    // TODO: do something
}

/*
BOOL InitCommonControlsEx(
  [in] const INITCOMMONCONTROLSEX *piccs
);
*/
fn InitCommonControlsEx(emu: &mut emu::Emu) {
    log_red!(emu, "comctl32!InitCommonControlsEx");
    // TODO: do something
    emu.regs_mut().rax = 1; // TRUE
}

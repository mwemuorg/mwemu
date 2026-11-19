use crate::api::dispatch_api;
use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64;

mod init_common_controls;
use init_common_controls::InitCommonControls;
mod init_common_controls_ex;
use init_common_controls_ex::InitCommonControlsEx;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    dispatch_api!(api, {
        "InitCommonControls" => InitCommonControls(emu),
        "InitCommonControlsEx" => InitCommonControlsEx(emu),
    },
        handled(()) => String::new(),
        fallback => fallback::unimplemented(addr, api, emu),
    )
}

/*
void InitCommonControls();
*/
/*
BOOL InitCommonControlsEx(
  [in] const INITCOMMONCONTROLSEX *piccs
);
*/

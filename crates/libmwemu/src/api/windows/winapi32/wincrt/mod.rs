use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::common::crt;
use crate::winapi::winapi32::kernel32;

mod set_invalid_parameter_handler;
use set_invalid_parameter_handler::set_invalid_parameter_handler;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    if crt::dispatch(api, emu) {
        return String::new();
    }
    match api {
        "_set_invalid_parameter_handler" => set_invalid_parameter_handler(emu),

        _ => {
            return fallback::unimplemented(addr, api, emu);
        }
    }

    String::new()
}

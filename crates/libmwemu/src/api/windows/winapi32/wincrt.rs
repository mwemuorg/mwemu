use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "_set_invalid_parameter_handler" => set_invalid_parameter_handler(emu),

        _ => {
            return fallback::unimplemented(addr, api, emu);
        }
    }

    String::new()
}

fn set_invalid_parameter_handler(emu: &mut emu::Emu) {
    log_red!(emu, "wincrt!_set_invalid_parameter_handler");
    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}

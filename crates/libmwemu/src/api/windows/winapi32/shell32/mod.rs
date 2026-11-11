use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;

mod launch;
pub use launch::*;
mod ui;
pub use ui::*;
mod objects;
pub use objects::*;
mod filesystem;
pub use filesystem::*;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    if launch::dispatch(api, emu)
        || ui::dispatch(api, emu)
        || objects::dispatch(api, emu)
        || filesystem::dispatch(api, emu)
    {
        return String::new();
    }

    fallback::unimplemented(addr, api, emu)
}

use crate::api::dispatch_api;
use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;

mod __deregister_frame_info;
use __deregister_frame_info::__deregister_frame_info;
mod __register_frame_info;
use __register_frame_info::__register_frame_info;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    dispatch_api!(api, {
        "__register_frame_info" => __register_frame_info(emu),
        "__deregister_frame_info" => __deregister_frame_info(emu),
    },
        handled(()) => String::new(),
        fallback => fallback::unimplemented(addr, api, emu),
    )
}

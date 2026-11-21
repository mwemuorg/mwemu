use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;
//use crate::endpoint;

mod path_file_exists;
use path_file_exists::PathFileExistsA;
use path_file_exists::PathFileExistsW;

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

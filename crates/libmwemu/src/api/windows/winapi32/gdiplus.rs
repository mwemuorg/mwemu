use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::fallback;
use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::policy::api_try;
use crate::winapi::winapi32::kernel32;

const DEFAULT_ALLOCATION_SIZE: u64 = 256;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split('!').next_back().unwrap_or(&api);
    match api {
        "GdipAlloc" => GdipAlloc(emu),
        _ => return fallback::unimplemented(addr, api, emu),
    }
    String::new()
}

fn GdipAlloc(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let size = api_try!(emu, call.arg(emu, 0));
    let address = if let Some(address) = emu.maps.alloc(size) {
        let _ = emu.maps.create_map(
            &format!("gdip_alloc_{address:x}"),
            address,
            size,
            Permission::READ_WRITE,
        );
        address
    } else {
        0
    };
    log_red!(emu, "gdiplus!GdipAlloc sz:{} => 0x{:x}", size, address);
    call.set_return(emu, address);
    call.cleanup_args(emu, 1);
}

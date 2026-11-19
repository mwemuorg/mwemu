use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn Shell_GetCachedImageIndex(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "shell32!Shell_GetCachedImageIndex arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("shell_getcachedimageindex");
    emu.set_last_error(0);
    call.set_return(emu, handle);
}

pub fn Shell_GetCachedImageIndexA(emu: &mut emu::Emu) {
    shell_get_cached_image_index(emu, "Shell_GetCachedImageIndexA", false);
}

pub fn Shell_GetCachedImageIndexW(emu: &mut emu::Emu) {
    shell_get_cached_image_index(emu, "Shell_GetCachedImageIndexW", true);
}

fn shell_get_cached_image_index(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    if wide {
        let handle = helper::handler_create("shell_getcachedimageindexw");
        emu.set_last_error(0);
        call.set_return(emu, handle);
    } else {
        let handle = helper::handler_create("shell_getcachedimageindexa");
        emu.set_last_error(0);
        call.set_return(emu, handle);
    }
}

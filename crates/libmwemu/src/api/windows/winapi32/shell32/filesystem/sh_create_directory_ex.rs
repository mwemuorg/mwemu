use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SHCreateDirectoryExA(emu: &mut emu::Emu) {
    sh_create_directory_ex(emu, "SHCreateDirectoryExA", false);
}

pub fn SHCreateDirectoryExW(emu: &mut emu::Emu) {
    sh_create_directory_ex(emu, "SHCreateDirectoryExW", true);
}

fn sh_create_directory_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    if wide {
        let handle = helper::handler_create("shcreatedirectoryexw");
        call.set_return(emu, handle);
        call.cleanup_args(emu, 3);
    } else {
        let handle = helper::handler_create("shcreatedirectoryexa");
        call.set_return(emu, handle);
        call.cleanup_args(emu, 3);
    }
}

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetFullPathNameA(emu: &mut emu::Emu) {
    get_full_path_name(emu, "GetFullPathNameA", false);
}

fn get_full_path_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let file_ptr = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg_u32(emu, 1));
    let buff = api_try!(emu, call.arg_u32(emu, 2));
    let path = api_try!(emu, call.arg_u32(emu, 3));

    let filename = strings::read_text(emu, file_ptr, wide);

    log_red!(emu, "kernel32!{api} file: {}", filename);

    call.cleanup_args(emu, 4);

    call.set_return(emu, 10);
}

pub fn GetFullPathNameW(emu: &mut emu::Emu) {
    get_full_path_name(emu, "GetFullPathNameW", true);
}

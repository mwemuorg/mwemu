use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetFinalPathNameByHandleA(emu: &mut emu::Emu) {
    get_final_path_name_by_handle(emu, "GetFinalPathNameByHandleA", false);
}

fn get_final_path_name_by_handle(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let buf = api_try!(emu, call.arg(emu, 1));
    let buf_size = api_try!(emu, call.arg(emu, 2));
    let uri = helper::handler_get_uri(handle);
    let path = uri.strip_prefix("file://").unwrap_or("C:\\unknown");
    log_red!(emu, "kernelbase!{api} h:0x{:x} => '{}'", handle, path);

    if buf != 0 && buf_size > 0 {
        strings::write_string(emu, buf, path, wide);
    }
    call.set_return(emu, path.len() as u64);
}

pub fn GetFinalPathNameByHandleW(emu: &mut emu::Emu) {
    get_final_path_name_by_handle(emu, "GetFinalPathNameByHandleW", true);
}

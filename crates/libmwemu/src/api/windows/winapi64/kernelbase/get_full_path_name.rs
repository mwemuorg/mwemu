use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetFullPathNameA(emu: &mut emu::Emu) {
    get_full_path_name(emu, "GetFullPathNameA", false);
}

fn get_full_path_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let buf_size = api_try!(emu, call.arg(emu, 1));
    let buf_ptr = api_try!(emu, call.arg(emu, 2));
    let filepart_ptr = api_try!(emu, call.arg(emu, 3));
    let path = strings::read_text(emu, path_ptr, wide);
    let full = format!("C:\\mwemu\\{}", path);
    if buf_ptr != 0 && buf_size > 0 {
        strings::write_string(emu, buf_ptr, &full, wide);
    }
    log_red!(emu, "kernelbase!{api} '{}' => '{}'", path, full);
    emu.set_last_error(0);
    call.set_return(emu, full.len() as u64);
}

pub fn GetFullPathNameW(emu: &mut emu::Emu) {
    get_full_path_name(emu, "GetFullPathNameW", true);
}

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn PathCchCanonicalize(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let out_ptr = api_try!(emu, call.arg(emu, 0));
    let cch_out = api_try!(emu, call.arg(emu, 1)) as usize;
    let path_ptr = api_try!(emu, call.arg(emu, 2));
    let path = emu.maps.read_wide_string(path_ptr);
    let mut parts: Vec<&str> = Vec::new();
    for part in path.split('\\') {
        match part {
            "." => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(part),
        }
    }
    let result_path = parts.join("\\");
    emu.maps.write_wide_string(out_ptr, &result_path);
    log_red!(emu, "kernelbase!{} '{}' => '{}'", api, path, result_path);
    let _h = helper::handler_create("dir_entry_current");
    call.set_return(emu, 0); // S_OK
}

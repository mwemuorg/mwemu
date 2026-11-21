use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn PathCchAddBackslash(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let cch_path = api_try!(emu, call.arg(emu, 1)) as usize;
    let path = emu.maps.read_wide_string(path_ptr);
    let mut result_path = path.clone();
    if !result_path.ends_with('\\') && !result_path.is_empty() {
        result_path.push('\\');
    }
    emu.maps.write_wide_string(path_ptr, &result_path);
    log_red!(emu, "kernelbase!{} '{}' => '{}'", api, path, result_path);
    call.set_return(emu, 0); // S_OK
}

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn PathIsUNCEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let server_ptr_ptr = api_try!(emu, call.arg(emu, 1));
    let path = if path_ptr != 0 {
        emu.maps.read_wide_string(path_ptr)
    } else {
        String::new()
    };
    let is_unc = path.starts_with("\\\\");
    if is_unc && server_ptr_ptr != 0 {
        emu.maps.write_qword(server_ptr_ptr, path_ptr + 4);
    }
    log_red!(emu, "kernelbase!PathIsUNCEx '{}' => {}", path, is_unc);
    emu.set_last_error(0);
    call.set_return(emu, if is_unc { 1 } else { 0 });
}

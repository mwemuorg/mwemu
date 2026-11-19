use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn FindFirstFileNameW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let find_data_ptr = api_try!(emu, call.arg(emu, 1));
    let path = emu.maps.read_wide_string(path_ptr);
    let hndl = helper::handler_create(&format!("find_{}", path));
    if find_data_ptr != 0 {
        emu.maps.write_dword(find_data_ptr, 0x80); // dwFileAttributes = NORMAL
    }
    log_red!(
        emu,
        "kernelbase!FindFirstFileNameW '{}' => 0x{:x}",
        path,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}

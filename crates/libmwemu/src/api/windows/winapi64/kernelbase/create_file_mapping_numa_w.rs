use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateFileMappingNumaW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let file_handle = api_try!(emu, call.arg(emu, 0));
    let security = api_try!(emu, call.arg(emu, 1));
    let protect = api_try!(emu, call.arg(emu, 2));
    let max_size_high = api_try!(emu, call.arg(emu, 3));
    let max_size_low = api_try!(emu, call.arg(emu, 5));
    let name_ptr = api_try!(emu, call.arg(emu, 6));
    let name = if name_ptr != 0 {
        emu.maps.read_wide_string(name_ptr)
    } else {
        String::from("(anon)")
    };
    let hndl = helper::handler_create(&format!("filemap_{}", name));
    log_red!(
        emu,
        "kernelbase!CreateFileMappingNumaW file:0x{:x} '{}' => 0x{:x}",
        file_handle,
        name,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}

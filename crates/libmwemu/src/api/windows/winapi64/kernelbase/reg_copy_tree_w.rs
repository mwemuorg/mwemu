use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn RegCopyTreeW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hkey_src = api_try!(emu, call.arg(emu, 0));
    let subkey_ptr = api_try!(emu, call.arg(emu, 1));
    let hkey_dst = api_try!(emu, call.arg(emu, 2));
    let subkey = if subkey_ptr != 0 {
        emu.maps.read_wide_string(subkey_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!RegCopyTreeW src:0x{:x} '{}' dst:0x{:x}",
        hkey_src,
        subkey,
        hkey_dst
    );
    emu.set_last_error(0);
    call.set_return(emu, 0); // ERROR_SUCCESS
}

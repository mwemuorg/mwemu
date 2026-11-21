use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn RegCreateKeyExA(emu: &mut emu::Emu) {
    reg_create_key_ex(emu, "RegCreateKeyExA", false);
}

fn reg_create_key_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg_u32(emu, 0));
    let subkey_ptr = api_try!(emu, call.arg(emu, 1));
    let reserved = api_try!(emu, call.arg_u32(emu, 2));
    let class_ptr = api_try!(emu, call.arg_u32(emu, 3));
    let options = api_try!(emu, call.arg_u32(emu, 4));
    let sam = api_try!(emu, call.arg_u32(emu, 5));
    let security = api_try!(emu, call.arg_u32(emu, 6));
    let result_ptr = api_try!(emu, call.arg(emu, 7));
    let disposition_ptr = api_try!(emu, call.arg(emu, 8));
    let subkey = strings::read_text(emu, subkey_ptr, wide);
    let new_handle = helper::handler_create(&subkey);
    if result_ptr != 0 {
        emu.maps.write_dword(result_ptr, new_handle as u32);
    }
    if disposition_ptr != 0 {
        emu.maps.write_dword(disposition_ptr, 2);
    }
    log_red!(
        emu,
        "advapi32!{api} hkey:0x{:x} '{}' => 0x{:x}",
        hkey,
        subkey,
        new_handle
    );
    call.cleanup_args(emu, 9);
    call.set_return(emu, 0);
}

pub fn RegCreateKeyExW(emu: &mut emu::Emu) {
    reg_create_key_ex(emu, "RegCreateKeyExW", true);
}

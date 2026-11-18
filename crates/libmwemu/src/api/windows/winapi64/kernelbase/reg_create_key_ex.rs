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
    let hkey = api_try!(emu, call.arg(emu, 0));
    let subkey_ptr = api_try!(emu, call.arg(emu, 1));
    let _reserved = api_try!(emu, call.arg(emu, 2));
    let _class = api_try!(emu, call.arg(emu, 3));
    let _options = api_try!(emu, call.arg(emu, 4));
    let _sam = api_try!(emu, call.arg(emu, 5));
    let _security = api_try!(emu, call.arg(emu, 6));
    let result_ptr = api_try!(emu, call.arg(emu, 7));
    let disp_ptr = api_try!(emu, call.arg(emu, 8));
    let subkey = strings::read_text(emu, subkey_ptr, wide);
    let hndl = helper::handler_create(&format!("regkey_{}", subkey));
    if result_ptr != 0 {
        emu.maps.write_qword(result_ptr, hndl);
    }
    if disp_ptr != 0 {
        emu.maps.write_dword(disp_ptr, 2); // REG_CREATED_NEW_KEY
    }
    log_red!(
        emu,
        "kernelbase!{api} hkey:0x{:x} '{}' => 0x{:x}",
        hkey,
        subkey,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}

pub fn RegCreateKeyExW(emu: &mut emu::Emu) {
    reg_create_key_ex(emu, "RegCreateKeyExW", true);
}

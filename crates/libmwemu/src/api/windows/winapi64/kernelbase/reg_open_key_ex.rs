use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn RegOpenKeyExA(emu: &mut emu::Emu) {
    reg_open_key_ex(emu, "RegOpenKeyExA", false);
}

fn reg_open_key_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let subkey_ptr = api_try!(emu, call.arg(emu, 1));
    let _reserved = api_try!(emu, call.arg(emu, 2));
    let _sam = api_try!(emu, call.arg(emu, 3));
    let result_ptr = api_try!(emu, call.arg(emu, 4));
    let subkey = strings::read_text(emu, subkey_ptr, wide);
    let hndl = helper::handler_create(&format!("regkey_{}", subkey));
    if result_ptr != 0 {
        emu.maps.write_qword(result_ptr, hndl);
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

pub fn RegOpenKeyExW(emu: &mut emu::Emu) {
    reg_open_key_ex(emu, "RegOpenKeyExW", true);
}

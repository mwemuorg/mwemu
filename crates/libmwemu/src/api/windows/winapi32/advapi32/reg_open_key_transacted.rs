use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn RegOpenKeyTransactedA(emu: &mut emu::Emu) {
    reg_open_key_transacted(emu, "RegOpenKeyTransactedA", false);
}

fn reg_open_key_transacted(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let subkey_ptr = api_try!(emu, call.arg(emu, 1));
    let reserved = api_try!(emu, call.arg(emu, 2));
    let sam = api_try!(emu, call.arg(emu, 3));
    let result_ptr = api_try!(emu, call.arg(emu, 4));
    let subkey = strings::read_text(emu, subkey_ptr, wide);
    let hndl = helper::handler_create(&format!("regkey_{}", subkey));
    if result_ptr != 0 {
        emu.maps.write_dword(result_ptr, hndl as u32);
    }
    log_red!(
        emu,
        "advapi32!{api} hkey:0x{:x} '{}' => 0x{:x}",
        hkey,
        subkey,
        hndl
    );
    call.set_return(emu, 0);
    call.cleanup_args(emu, 7);
}

pub fn RegOpenKeyTransactedW(emu: &mut emu::Emu) {
    reg_open_key_transacted(emu, "RegOpenKeyTransactedW", true);
}

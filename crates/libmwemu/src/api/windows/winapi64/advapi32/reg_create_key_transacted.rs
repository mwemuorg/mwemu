use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn RegCreateKeyTransactedA(emu: &mut emu::Emu) {
    reg_create_key_transacted(emu, "RegCreateKeyTransactedA", false);
}

fn reg_create_key_transacted(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let subkey_ptr = api_try!(emu, call.arg(emu, 1));
    let subkey = strings::read_text(emu, subkey_ptr, wide);
    let hndl = helper::handler_create(&format!("regkey_{}", subkey));
    log_red!(
        emu,
        "advapi32!{api} hkey:0x{:x} '{}' => 0x{:x}",
        hkey,
        subkey,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}

pub fn RegCreateKeyTransactedW(emu: &mut emu::Emu) {
    reg_create_key_transacted(emu, "RegCreateKeyTransactedW", true);
}

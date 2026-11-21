use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptGetUserKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hprov = api_try!(emu, call.arg(emu, 0));
    let key_spec = api_try!(emu, call.arg(emu, 1));
    let ph_user_key = api_try!(emu, call.arg(emu, 2));
    let handle = helper::handler_create("crypt_user_key");
    crypto_state::create_key(handle, 0x6801, 128);
    log_red!(
        emu,
        "advapi32!CryptGetUserKey spec:{} => 0x{:x}",
        key_spec,
        handle
    );
    if ph_user_key != 0 {
        emu.maps.write_qword(ph_user_key, handle);
    }
    call.set_return(emu, 1);
}

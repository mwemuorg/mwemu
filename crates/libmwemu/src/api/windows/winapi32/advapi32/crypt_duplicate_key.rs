use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptDuplicateKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let _reserved = api_try!(emu, call.arg_u32(emu, 1));
    let _flags = api_try!(emu, call.arg_u32(emu, 2));
    let ph_key = api_try!(emu, call.arg(emu, 3));
    let new_handle = helper::handler_create("crypt_key_dup");
    crypto_state::create_key(new_handle, 0x6801, 128);
    log_red!(
        emu,
        "advapi32!CryptDuplicateKey src:0x{:x} => 0x{:x}",
        hkey,
        new_handle
    );
    if ph_key != 0 {
        emu.maps.write_dword(ph_key, new_handle as u32);
    }
    call.cleanup_args(emu, 4);
    call.set_return(emu, 1);
}

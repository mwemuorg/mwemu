use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptGenKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_prov = api_try!(emu, call.arg(emu, 0));
    let algid = api_try!(emu, call.arg_u32(emu, 1));
    let flags = api_try!(emu, call.arg_u32(emu, 2));
    let h_key_ptr = api_try!(emu, call.arg(emu, 3));
    call.cleanup_args(emu, 4);
    log_red!(
        emu,
        "crypt32!CryptGenKey prov: 0x{:x} alg: 0x{:x} flags: 0x{:x}",
        h_prov,
        algid,
        flags
    );
    let h_key = helper::handler_create(&format!("cryptkey:0x{:x}", algid));
    let key_bits = flags >> 16;
    crypto_state::create_key(h_key, algid, key_bits);
    if h_key_ptr != 0 {
        emu.maps.write_dword(h_key_ptr, h_key as u32);
    }
    call.set_return(emu, 1); // TRUE
}

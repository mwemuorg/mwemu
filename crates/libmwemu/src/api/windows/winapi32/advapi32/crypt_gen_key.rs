use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::crypto::*;

pub(super) fn CryptGenKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hprov = api_try!(emu, call.arg_u32(emu, 0));
    let algid = api_try!(emu, call.arg_u32(emu, 1));
    let flags = api_try!(emu, call.arg_u32(emu, 2));
    let hkey_ptr = api_try!(emu, call.arg(emu, 3));

    let uri = format!("cryptkey://{}", get_cryptoalgorithm_name(algid));
    let hndl = helper::handler_create(&uri);
    assert!(hndl < 0x00000001_00000000);

    let key_bits = flags >> 16;
    crypto_state::create_key(hndl, algid, key_bits);

    if hkey_ptr != 0 {
        emu.maps.write_dword(hkey_ptr, hndl as u32);
    }

    log_red!(
        emu,
        "advapi32!CryptGenKey {} =0x{:x}",
        get_cryptoalgorithm_name(algid),
        hndl
    );

    call.cleanup_args(emu, 4);
    call.set_return(emu, 1);
}

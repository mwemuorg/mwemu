use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::crypto::*;

pub(super) fn CryptCreateHash(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hprov = api_try!(emu, call.arg_u32(emu, 0));
    let algid = api_try!(emu, call.arg_u32(emu, 1));
    let hkey = api_try!(emu, call.arg_u32(emu, 2));
    let flags = api_try!(emu, call.arg_u32(emu, 3));
    let hash_ptr = api_try!(emu, call.arg(emu, 4));

    let uri = format!("crypthash://{}", get_cryptoalgorithm_name(algid));
    let hndl = helper::handler_create(&uri);
    assert!(hndl < 0x00000001_00000000);

    crypto_state::create_hash(hndl, algid);
    emu.maps.write_dword(hash_ptr, hndl as u32);

    log_red!(
        emu,
        "advapi32!CryptCreateHash {} =0x{:x}",
        get_cryptoalgorithm_name(algid),
        hndl
    );

    call.cleanup_args(emu, 5);
    call.set_return(emu, 1);
}

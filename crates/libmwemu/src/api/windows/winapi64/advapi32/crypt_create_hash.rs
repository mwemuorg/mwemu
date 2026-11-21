use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::crypto::*;

pub fn CryptCreateHash(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hprov = api_try!(emu, call.arg(emu, 0));
    let alg_id = api_try!(emu, call.arg_u32(emu, 1));
    let _hkey = api_try!(emu, call.arg(emu, 2));
    let _flags = api_try!(emu, call.arg_u32(emu, 3));
    let ph_hash = api_try!(emu, call.arg(emu, 4));

    let alg_name = get_cryptoalgorithm_name(alg_id);
    let uri = format!("crypthash://{}", alg_name);
    let hndl = helper::handler_create(&uri);

    crypto_state::create_hash(hndl, alg_id);

    if ph_hash != 0 {
        emu.maps.write_qword(ph_hash, hndl);
    }

    log_red!(
        emu,
        "advapi32!CryptCreateHash alg={} =0x{:x}",
        alg_name,
        hndl
    );

    call.set_return(emu, 1);
}

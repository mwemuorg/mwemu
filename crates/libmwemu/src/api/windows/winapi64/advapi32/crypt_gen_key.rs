use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::crypto::*;

pub fn CryptGenKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hprov = api_try!(emu, call.arg(emu, 0));
    let alg_id = api_try!(emu, call.arg_u32(emu, 1));
    let flags = api_try!(emu, call.arg(emu, 2));
    let ph_key = api_try!(emu, call.arg(emu, 3));

    let alg_name = get_cryptoalgorithm_name(alg_id);
    let uri = format!("cryptkey://{}", alg_name);
    let hndl = helper::handler_create(&uri);

    let key_bits = (flags >> 16) as u32;
    crypto_state::create_key(hndl, alg_id, key_bits);

    if ph_key != 0 {
        emu.maps.write_qword(ph_key, hndl);
    }

    log_red!(emu, "advapi32!CryptGenKey alg={} =0x{:x}", alg_name, hndl);

    call.set_return(emu, 1);
}

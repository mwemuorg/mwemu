use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn BCryptGenerateSymmetricKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_algorithm = api_try!(emu, call.arg(emu, 0));
    let ph_key = api_try!(emu, call.arg(emu, 1));
    let _pb_key_object = api_try!(emu, call.arg(emu, 2));
    let _cb_key_object = api_try!(emu, call.arg(emu, 3));
    let pb_secret = api_try!(emu, call.arg(emu, 4));
    let cb_secret = api_try!(emu, call.arg(emu, 5)) as usize;
    let _flags = api_try!(emu, call.arg(emu, 6));

    let algo_uri = helper::handler_get_uri(h_algorithm);
    let algo_name = algo_uri.strip_prefix("bcrypt://").unwrap_or(&algo_uri);

    let hndl = helper::handler_create(&format!("bcryptkey://{}", algo_name));

    let calg = match algo_name.to_uppercase().as_str() {
        "AES" => 0x00006611,
        "RC4" => 0x00006801,
        "3DES" => 0x00006603,
        _ => 0,
    };
    let key_bits = (cb_secret * 8) as u32;
    if calg != 0 {
        crypto_state::create_key(hndl, calg, key_bits);
    }
    if pb_secret != 0 && cb_secret > 0 {
        let key_data = emu.maps.read_bytes(pb_secret, cb_secret.min(256)).to_vec();
        crypto_state::set_key_data(hndl, &key_data);
    }

    if ph_key != 0 {
        emu.maps.write_qword(ph_key, hndl);
    }

    log_red!(
        emu,
        "bcrypt!BCryptGenerateSymmetricKey algo={} key_len={} =0x{:x}",
        algo_name,
        cb_secret,
        hndl
    );

    call.set_return(emu, STATUS_SUCCESS);
}

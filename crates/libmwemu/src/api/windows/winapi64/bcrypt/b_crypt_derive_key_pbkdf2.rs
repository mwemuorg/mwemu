use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn BCryptDeriveKeyPBKDF2(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_prf = api_try!(emu, call.arg(emu, 0));
    let pb_password = api_try!(emu, call.arg(emu, 1));
    let cb_password = api_try!(emu, call.arg(emu, 2)) as usize;
    let pb_salt = api_try!(emu, call.arg(emu, 3));
    let cb_salt = api_try!(emu, call.arg(emu, 4)) as usize;
    let iterations = api_try!(emu, call.arg(emu, 5));
    let pb_derived_key = api_try!(emu, call.arg(emu, 6));
    let cb_derived_key = api_try!(emu, call.arg(emu, 7)) as usize;

    if pb_derived_key != 0 && cb_derived_key > 0 {
        let pwd = if pb_password != 0 && cb_password > 0 {
            emu.maps
                .read_bytes(pb_password, cb_password.min(4096))
                .to_vec()
        } else {
            vec![]
        };
        let salt = if pb_salt != 0 && cb_salt > 0 {
            emu.maps.read_bytes(pb_salt, cb_salt.min(4096)).to_vec()
        } else {
            vec![]
        };
        let uri = helper::handler_get_uri(h_prf);
        let alg_id = if uri.contains("SHA1") || uri.contains("sha1") {
            0x00008004u32
        } else {
            0x0000800cu32
        };
        let mut key = vec![0u8; cb_derived_key];
        crypto_state::pbkdf2_derive(alg_id, &pwd, &salt, iterations as u32, &mut key);
        emu.maps.write_bytes(pb_derived_key, &key);
    }

    log_red!(
        emu,
        "bcrypt!BCryptDeriveKeyPBKDF2 pwd_len={} salt_len={} iters={} key_len={}",
        cb_password,
        cb_salt,
        iterations,
        cb_derived_key
    );

    call.set_return(emu, STATUS_SUCCESS);
}

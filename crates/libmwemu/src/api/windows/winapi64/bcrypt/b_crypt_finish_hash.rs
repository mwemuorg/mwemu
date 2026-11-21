use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn BCryptFinishHash(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_hash = api_try!(emu, call.arg(emu, 0));
    let pb_output = api_try!(emu, call.arg(emu, 1));
    let cb_output = api_try!(emu, call.arg(emu, 2)) as usize;
    let flags = api_try!(emu, call.arg(emu, 3));

    if let Some(digest) = crypto_state::get_hash_param(h_hash, crypto_state::HP_HASHVAL) {
        if pb_output != 0 {
            let write_len = digest.len().min(cb_output);
            emu.maps.write_bytes(pb_output, &digest[..write_len]);
        }
        log_red!(
            emu,
            "bcrypt!BCryptFinishHash hash=0x{:x} digest_len={}",
            h_hash,
            digest.len()
        );
    } else {
        log_red!(
            emu,
            "bcrypt!BCryptFinishHash hash=0x{:x} (no state)",
            h_hash
        );
    }

    call.set_return(emu, STATUS_SUCCESS);
}

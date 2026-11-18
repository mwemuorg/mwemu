use super::bcrypt_algo_to_calg;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn BCryptCreateHash(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_algorithm = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let ph_hash = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let _pb_hash_object = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let _cb_hash_object = api_try!(emu, call.arg_u32(emu, 3));
    let _pb_secret = api_try!(emu, call.arg_u32(emu, 4)) as u64;
    let _cb_secret = api_try!(emu, call.arg_u32(emu, 5));
    let _flags = api_try!(emu, call.arg_u32(emu, 6));

    let algo_uri = helper::handler_get_uri(h_algorithm);
    let algo_name = algo_uri.strip_prefix("bcrypt://").unwrap_or(&algo_uri);
    let calg = bcrypt_algo_to_calg(algo_name);

    let hndl = helper::handler_create(&format!("bcrypthash://{}", algo_name));
    if calg != 0 {
        crypto_state::create_hash(hndl, calg);
    }

    if ph_hash != 0 {
        emu.maps.write_dword(ph_hash, hndl as u32);
    }

    log_red!(
        emu,
        "bcrypt!BCryptCreateHash algo={} =0x{:x}",
        algo_name,
        hndl
    );

    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 7);
}

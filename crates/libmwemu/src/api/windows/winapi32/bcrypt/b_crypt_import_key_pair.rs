use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn BCryptImportKeyPair(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_algorithm = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let _h_import_key = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let _psz_blob_type = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let ph_key = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    let _pb_input = api_try!(emu, call.arg_u32(emu, 4)) as u64;
    let _cb_input = api_try!(emu, call.arg_u32(emu, 5));

    let algo_uri = helper::handler_get_uri(h_algorithm);
    let algo_name = algo_uri.strip_prefix("bcrypt://").unwrap_or(&algo_uri);
    let hndl = helper::handler_create(&format!("bcryptkey://{}", algo_name));

    if ph_key != 0 {
        emu.maps.write_dword(ph_key, hndl as u32);
    }

    log_red!(
        emu,
        "bcrypt!BCryptImportKeyPair algo={} =0x{:x}",
        algo_name,
        hndl
    );

    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 7);
}

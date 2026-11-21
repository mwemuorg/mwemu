use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn BCryptOpenAlgorithmProvider(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let out_handle = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let alg_id_ptr = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let algo = emu.maps.read_wide_string(alg_id_ptr);
    log_red!(emu, "bcrypt!BCryptOpenAlgorithmProvider algo: {}", algo);
    if out_handle != 0 {
        let hndl = helper::handler_create(&format!("bcrypt://{}", algo));
        emu.maps.write_dword(out_handle, hndl as u32);
    }
    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 4);
}

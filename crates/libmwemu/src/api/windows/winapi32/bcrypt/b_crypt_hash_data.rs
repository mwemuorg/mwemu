use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn BCryptHashData(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_hash = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let pb_input = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let cb_input = api_try!(emu, call.arg_u32(emu, 2)) as usize;
    let _flags = api_try!(emu, call.arg_u32(emu, 3));

    let len = cb_input.min(0x100_0000);
    if pb_input != 0 && len > 0 {
        let data = emu.maps.read_bytes(pb_input, len);
        crypto_state::hash_data(h_hash, &data);
    }

    log_red!(
        emu,
        "bcrypt!BCryptHashData hash=0x{:x} len={}",
        h_hash,
        cb_input
    );

    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 4);
}

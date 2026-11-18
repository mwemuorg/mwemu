use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GlobalAddAtomA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lp_string = api_try!(emu, call.arg(emu, 0));

    let atom_name = emu.maps.read_string(lp_string);

    log_red!(emu, "kernel32!GlobalAddAtomA name: `{}`", atom_name);

    call.cleanup_args(emu, 1);

    // TODO: implement real atom table
    call.set_return(emu, 0xC001); // Return a dummy atom
}

pub fn GlobalAddAtomW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lpString = api_try!(emu, call.arg_u32(emu, 0));

    let lpString = emu.maps.read_wide_string(lpString as u64);

    log_red!(emu, "kernel32!GlobalAddAtomW {}", lpString);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 1);
}

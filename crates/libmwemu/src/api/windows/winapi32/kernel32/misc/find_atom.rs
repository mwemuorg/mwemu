use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn FindAtomA(emu: &mut emu::Emu) {
    find_atom(emu, "FindAtomA", false);
}

fn find_atom(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpString = api_try!(emu, call.arg_u32(emu, 0));

    let lpString = strings::read_text(emu, lpString as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpString);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 1);
}

pub fn FindAtomW(emu: &mut emu::Emu) {
    find_atom(emu, "FindAtomW", true);
}

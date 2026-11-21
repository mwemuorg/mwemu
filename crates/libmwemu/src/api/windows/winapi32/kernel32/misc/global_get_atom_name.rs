use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GlobalGetAtomNameA(emu: &mut emu::Emu) {
    global_get_atom_name(emu, "GlobalGetAtomNameA");
}

fn global_get_atom_name(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let _nAtom = api_try!(emu, call.arg_u32(emu, 0));
    let _lpBuffer = api_try!(emu, call.arg_u32(emu, 1));
    let _nSize = api_try!(emu, call.arg_u32(emu, 2));

    log_red!(emu, "kernel32!{api}");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn GlobalGetAtomNameW(emu: &mut emu::Emu) {
    global_get_atom_name(emu, "GlobalGetAtomNameW");
}

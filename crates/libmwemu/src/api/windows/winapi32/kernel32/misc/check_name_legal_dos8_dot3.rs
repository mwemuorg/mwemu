use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn CheckNameLegalDOS8Dot3A(emu: &mut emu::Emu) {
    check_name_legal_dos8_dot3(emu, "CheckNameLegalDOS8Dot3A", false);
}

fn check_name_legal_dos8_dot3(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpName = api_try!(emu, call.arg_u32(emu, 0));
    let lpOemName = api_try!(emu, call.arg_u32(emu, 1));
    let OemNameSize = api_try!(emu, call.arg_u32(emu, 2));
    let pbNameContainsSpaces = api_try!(emu, call.arg_u32(emu, 3));
    let pbNameLegal = api_try!(emu, call.arg_u32(emu, 4));

    let lpName = strings::read_text(emu, lpName as u64, wide);
    let lpOemName = strings::read_text(emu, lpOemName as u64, wide);
    let OemNameSize = strings::read_text(emu, OemNameSize as u64, wide);
    let pbNameContainsSpaces = strings::read_text(emu, pbNameContainsSpaces as u64, wide);
    let pbNameLegal = strings::read_text(emu, pbNameLegal as u64, wide);

    log_red!(
        emu,
        "kernel32!{api} {} {} {} {} {}",
        lpName,
        lpOemName,
        OemNameSize,
        pbNameContainsSpaces,
        pbNameLegal
    );

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 5);
}

pub fn CheckNameLegalDOS8Dot3W(emu: &mut emu::Emu) {
    check_name_legal_dos8_dot3(emu, "CheckNameLegalDOS8Dot3W", true);
}

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn MoveFileTransactedA(emu: &mut emu::Emu) {
    move_file_transacted(emu, "MoveFileTransactedA", false);
}

fn move_file_transacted(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpExistingFileName = api_try!(emu, call.arg_u32(emu, 0));
    let lpNewFileName = api_try!(emu, call.arg_u32(emu, 1));
    let _lpProgressRoutine = api_try!(emu, call.arg_u32(emu, 2));
    let _lpData = api_try!(emu, call.arg_u32(emu, 3));
    let _dwFlags = api_try!(emu, call.arg_u32(emu, 4));
    let _hTransaction = api_try!(emu, call.arg_u32(emu, 5));

    let lpExistingFileName = strings::read_text(emu, lpExistingFileName as u64, wide);
    let lpNewFileName = strings::read_text(emu, lpNewFileName as u64, wide);

    log_red!(
        emu,
        "kernel32!{api} {} {}",
        lpExistingFileName,
        lpNewFileName
    );

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 6);
}

pub fn MoveFileTransactedW(emu: &mut emu::Emu) {
    move_file_transacted(emu, "MoveFileTransactedW", true);
}

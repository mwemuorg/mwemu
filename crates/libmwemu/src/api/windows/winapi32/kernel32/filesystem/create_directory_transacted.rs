use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn CreateDirectoryTransactedA(emu: &mut emu::Emu) {
    create_directory_transacted(emu, "CreateDirectoryTransactedA");
}

fn create_directory_transacted(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let _lpTemplateDirectory = api_try!(emu, call.arg_u32(emu, 0));
    let _lpNewDirectory = api_try!(emu, call.arg_u32(emu, 1));
    let _lpSecurityAttributes = api_try!(emu, call.arg_u32(emu, 2));
    let _hTransaction = api_try!(emu, call.arg_u32(emu, 3));

    log_red!(emu, "kernel32!{api}");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 4);
}

pub fn CreateDirectoryTransactedW(emu: &mut emu::Emu) {
    create_directory_transacted(emu, "CreateDirectoryTransactedW");
}

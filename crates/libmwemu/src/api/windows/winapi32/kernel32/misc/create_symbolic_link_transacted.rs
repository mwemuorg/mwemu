use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn CreateSymbolicLinkTransactedA(emu: &mut emu::Emu) {
    create_symbolic_link_transacted(emu, "CreateSymbolicLinkTransactedA", false);
}

fn create_symbolic_link_transacted(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpSymlinkFileName = api_try!(emu, call.arg_u32(emu, 0));
    let lpTargetFileName = api_try!(emu, call.arg_u32(emu, 1));
    let _dwFlags = api_try!(emu, call.arg_u32(emu, 2));
    let _hTransaction = api_try!(emu, call.arg_u32(emu, 3));

    let lpSymlinkFileName = strings::read_text(emu, lpSymlinkFileName as u64, wide);
    let lpTargetFileName = strings::read_text(emu, lpTargetFileName as u64, wide);

    log_red!(
        emu,
        "kernel32!{api} {} {}",
        lpSymlinkFileName,
        lpTargetFileName
    );

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 4);
}

pub fn CreateSymbolicLinkTransactedW(emu: &mut emu::Emu) {
    create_symbolic_link_transacted(emu, "CreateSymbolicLinkTransactedW", true);
}

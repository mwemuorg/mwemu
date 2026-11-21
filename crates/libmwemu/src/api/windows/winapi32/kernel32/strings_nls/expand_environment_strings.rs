use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn ExpandEnvironmentStringsA(emu: &mut emu::Emu) {
    expand_environment_strings(emu, "ExpandEnvironmentStringsA", false);
}

fn expand_environment_strings(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let src_ptr = api_try!(emu, call.arg(emu, 0));
    let dst_ptr = api_try!(emu, call.arg(emu, 1));
    let size = api_try!(emu, call.arg_u32(emu, 2));

    let src = strings::read_text(emu, src_ptr, wide);

    log_red!(emu, "kernel32!{api} `{}`", src);

    call.cleanup_args(emu, 3);

    call.set_return(emu, 1);

    //TODO: implement expand
}

pub fn ExpandEnvironmentStringsW(emu: &mut emu::Emu) {
    expand_environment_strings(emu, "ExpandEnvironmentStringsW", true);
}

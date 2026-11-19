use super::expand_env_vars;
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
    let size = api_try!(emu, call.arg(emu, 2));
    let src = strings::read_text(emu, src_ptr, wide);
    let expanded = expand_env_vars(&src);
    if dst_ptr != 0 && size > expanded.len() as u64 {
        strings::write_string(emu, dst_ptr, &expanded, wide);
    }
    log_red!(emu, "kernelbase!{api} '{}' => '{}'", src, expanded);
    call.set_return(emu, expanded.len() as u64 + 1);
}

pub fn ExpandEnvironmentStringsW(emu: &mut emu::Emu) {
    expand_environment_strings(emu, "ExpandEnvironmentStringsW", true);
}

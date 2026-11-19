use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn SetEnvironmentVariableA(emu: &mut emu::Emu) {
    set_environment_variable(emu, "SetEnvironmentVariableA", false);
}

fn set_environment_variable(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name_ptr = api_try!(emu, call.arg(emu, 0));
    let value_ptr = api_try!(emu, call.arg(emu, 1));
    let name = strings::read_text(emu, name_ptr, wide);
    let value = if value_ptr != 0 {
        strings::read_text(emu, value_ptr, wide)
    } else {
        String::from("(null)")
    };
    log_red!(emu, "kernelbase!{api} '{}' = '{}'", name, value);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn SetEnvironmentVariableW(emu: &mut emu::Emu) {
    set_environment_variable(emu, "SetEnvironmentVariableW", true);
}

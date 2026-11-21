use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn SystemParametersInfoA(emu: &mut emu::Emu) {
    system_parameters_info(emu, "SystemParametersInfoA", false);
}

pub fn SystemParametersInfoW(emu: &mut emu::Emu) {
    system_parameters_info(emu, "SystemParametersInfoW", true);
}

fn system_parameters_info(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    if wide {
        let a0 = api_try!(emu, call.arg_u32(emu, 0)) as u64;
        let a1 = api_try!(emu, call.arg_u32(emu, 1)) as u64;
        log_red!(emu, "user32!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
        emu.set_last_error(0);
    } else {
        let action = api_try!(emu, call.arg_u32(emu, 0));
        log_red!(emu, "user32!{api} uiAction: 0x{:x}", action);
    }

    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}

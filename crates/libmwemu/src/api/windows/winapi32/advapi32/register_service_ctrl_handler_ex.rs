use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn RegisterServiceCtrlHandlerExA(emu: &mut emu::Emu) {
    register_service_ctrl_handler_ex(emu, "RegisterServiceCtrlHandlerExA");
}

fn register_service_ctrl_handler_ex(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg_u32(emu, 0));
    let arg2 = api_try!(emu, call.arg_u32(emu, 1));
    let arg3 = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(
        emu,
        "advapi32!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    call.cleanup_args(emu, 3);
    let hndl = helper::handler_create(&format!("advapi32://{api}"));
    call.set_return(emu, hndl);
}

pub fn RegisterServiceCtrlHandlerExW(emu: &mut emu::Emu) {
    register_service_ctrl_handler_ex(emu, "RegisterServiceCtrlHandlerExW");
}

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn StartServiceCtrlDispatcherA(emu: &mut emu::Emu) {
    start_service_ctrl_dispatcher(emu, "StartServiceCtrlDispatcherA");
}

fn start_service_ctrl_dispatcher(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let service_table_entry_ptr = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "advapi32!{api} ptr:0x{:x}", service_table_entry_ptr);

    call.set_return(emu, 1);
}

pub(super) fn StartServiceCtrlDispatcherW(emu: &mut emu::Emu) {
    start_service_ctrl_dispatcher(emu, "StartServiceCtrlDispatcherW");
}

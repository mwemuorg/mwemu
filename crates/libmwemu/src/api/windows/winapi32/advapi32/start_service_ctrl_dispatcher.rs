use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn StartServiceCtrlDispatcherA(emu: &mut emu::Emu) {
    start_service_ctrl_dispatcher(emu, "StartServiceCtrlDispatcherA");
}

pub(super) fn StartServiceCtrlDispatcherW(emu: &mut emu::Emu) {
    start_service_ctrl_dispatcher(emu, "StartServiceCtrlDispatcherW");
}

fn start_service_ctrl_dispatcher(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let service_table_entry_ptr = api_try!(emu, call.arg_u32(emu, 0));
    /*
    let service_name = emu.maps.read_dword(service_table_entry_ptr as u64)
        .unwrap_or(0);
    let service_name = emu.maps.read_dword((service_table_entry_ptr+4) as u64)
        .unwrap_or(0);*/

    log_red!(emu, "advapi32!{api}");

    call.cleanup_args(emu, 1);
    call.set_return(emu, 1);
}

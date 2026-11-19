use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn CreateJobObjectA(emu: &mut emu::Emu) {
    create_job_object(emu, "CreateJobObjectA");
}

fn create_job_object(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 2);
    log_red!(emu, "kernel32!{api}");
    call.set_return(emu, helper::handler_create(api));
}

pub fn CreateJobObjectW(emu: &mut emu::Emu) {
    create_job_object(emu, "CreateJobObjectW");
}

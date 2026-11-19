use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn StartServiceA(emu: &mut emu::Emu) {
    start_service(emu, "StartServiceA");
}

fn start_service(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let hsvc = api_try!(emu, call.arg(emu, 0));
    let num_args = api_try!(emu, call.arg(emu, 1));
    let args_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "advapi32!{api} svc:0x{:x} nargs:{}", hsvc, num_args);

    for _ in 0..3 {}
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn StartServiceW(emu: &mut emu::Emu) {
    start_service(emu, "StartServiceW");
}

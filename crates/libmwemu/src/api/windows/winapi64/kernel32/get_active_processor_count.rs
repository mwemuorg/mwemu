use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetActiveProcessorCount(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let group = api_try!(emu, call.arg(emu, 0));
    let count: u64 = 4;
    log_red!(
        emu,
        "kernel32!GetActiveProcessorCount group:{} => {}",
        group,
        count
    );
    emu.set_last_error(0);
    call.set_return(emu, count);
}

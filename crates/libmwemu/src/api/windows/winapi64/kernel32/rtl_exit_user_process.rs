use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn RtlExitUserProcess(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let exit_code = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ntdll!RtlExitUserProcess code:{}", exit_code);
    emu.stop();
}

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy::api_try;

pub fn SystemFunction036(emu: &mut Emu) {
    system_random(emu, "advapi32");
}

pub(crate) fn system_random(emu: &mut Emu, library: &str) {
    let call = WinApi(Stdcall);
    let buffer = api_try!(emu, call.arg(emu, 0));
    let length = api_try!(emu, call.arg_u32(emu, 1));
    let ok = emu.fill_random(buffer, length.into());
    log_red!(
        emu,
        "{library}!SystemFunction036 buffer={buffer:#x} length={length} result={ok}"
    );
    call.set_return(emu, ok.into());
    call.cleanup_args(emu, 2);
}

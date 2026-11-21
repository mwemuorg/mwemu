use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SafeArraySetRecordInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sa = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "oleaut32!SafeArraySetRecordInfo 0x{:x}", sa);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}

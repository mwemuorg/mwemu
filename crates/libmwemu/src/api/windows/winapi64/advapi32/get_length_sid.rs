use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetLengthSid(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sid_ptr = api_try!(emu, call.arg(emu, 0));
    let sub_count = if sid_ptr != 0 {
        emu.maps.read_byte(sid_ptr + 1).unwrap_or(5) as u64
    } else {
        5
    };
    let len = 8 + sub_count * 4;
    log_red!(emu, "advapi32!GetLengthSid sid:0x{:x} => {}", sid_ptr, len);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, len);
}

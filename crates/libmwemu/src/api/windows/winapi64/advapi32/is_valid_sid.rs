use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn IsValidSid(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sid_ptr = api_try!(emu, call.arg(emu, 0));
    let valid = if sid_ptr != 0 {
        let revision = emu.maps.read_byte(sid_ptr).unwrap_or(0);
        let sub_count = emu.maps.read_byte(sid_ptr + 1).unwrap_or(0);
        revision == 1 && sub_count <= 15
    } else {
        false
    };
    log_red!(emu, "advapi32!IsValidSid sid:0x{:x} => {}", sid_ptr, valid);
    call.set_return(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, if valid { 1 } else { 0 });
}

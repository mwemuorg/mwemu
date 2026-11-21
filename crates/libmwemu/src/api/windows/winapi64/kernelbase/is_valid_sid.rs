use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn IsValidSid(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_sid = api_try!(emu, call.arg(emu, 0));
    let valid = if p_sid != 0 {
        let revision = emu.maps.read_byte(p_sid).unwrap_or(0);
        let sub_count = emu.maps.read_byte(p_sid + 1).unwrap_or(0);
        if revision == 1 && sub_count <= 15 {
            1u64
        } else {
            0
        }
    } else {
        0
    };
    log_red!(emu, "kernelbase!IsValidSid sid:0x{:x} => {}", p_sid, valid);
    call.set_return(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, valid);
}

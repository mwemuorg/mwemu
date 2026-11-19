use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn IsWellKnownSid(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sid = api_try!(emu, call.arg(emu, 0));
    let well_known_type = api_try!(emu, call.arg_u32(emu, 1));
    let result = if sid != 0 {
        let revision = emu.maps.read_byte(sid).unwrap_or(0);
        let sub_count = emu.maps.read_byte(sid + 1).unwrap_or(0);
        if revision == 1 && sub_count > 0 {
            let sub0 = emu.maps.read_dword(sid + 8).unwrap_or(0);
            match well_known_type {
                0 => sub0 == 0,
                1 => sub0 == 0,
                22 => sub0 == 544,
                _ => false,
            }
        } else {
            false
        }
    } else {
        false
    };
    log_red!(
        emu,
        "kernelbase!IsWellKnownSid sid:0x{:x} type:{} => {}",
        sid,
        well_known_type,
        result
    );
    call.set_return(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, if result { 1 } else { 0 });
}

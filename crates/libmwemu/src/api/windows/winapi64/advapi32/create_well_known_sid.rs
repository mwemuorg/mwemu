use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CreateWellKnownSid(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sid_type = api_try!(emu, call.arg_u32(emu, 0));
    let _domain_sid = api_try!(emu, call.arg(emu, 1));
    let sid_ptr = api_try!(emu, call.arg(emu, 2));
    let cb_sid_ptr = api_try!(emu, call.arg(emu, 3));
    let sid_bytes: [u8; 12] = [1, 1, 0, 0, 0, 0, 0, 5, 0x12, 0, 0, 0];
    if cb_sid_ptr != 0 {
        emu.maps.write_dword(cb_sid_ptr, 12);
    }
    if sid_ptr != 0 {
        emu.maps.write_bytes(sid_ptr, &sid_bytes);
    }
    log_red!(
        emu,
        "advapi32!CreateWellKnownSid type:{} sid:0x{:x}",
        sid_type,
        sid_ptr
    );
    call.set_return(emu, 1);
}

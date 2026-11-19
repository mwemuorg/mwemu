use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CreateWellKnownSid(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let well_known_sid_type = api_try!(emu, call.arg_u32(emu, 0));
    let domain_sid = api_try!(emu, call.arg(emu, 1));
    let p_sid = api_try!(emu, call.arg(emu, 2));
    let cb_sid = api_try!(emu, call.arg(emu, 3));

    log_red!(
        emu,
        "kernelbase!CreateWellKnownSid type:{} buf:0x{:x}",
        well_known_sid_type,
        p_sid
    );

    let sid_size: u32 = 28; // 8 + 5*4 = typical SID
    if cb_sid != 0 {
        let provided = emu.maps.read_dword(cb_sid).unwrap_or(0);
        emu.maps.write_dword(cb_sid, sid_size);
        if provided < sid_size {
            emu.set_last_error(122); // ERROR_INSUFFICIENT_BUFFER
            call.set_return(emu, 0);
        } else if p_sid != 0 {
            // Write a mock SID: revision=1, sub_auth_count=5, NT authority
            emu.maps.write_byte(p_sid, 1); // revision
            emu.maps.write_byte(p_sid + 1, 5); // sub-authority count
            // identifier authority: NT Authority {0,0,0,0,0,5}
            for i in 0..5u64 {
                emu.maps.write_byte(p_sid + 2 + i, 0);
            }
            emu.maps.write_byte(p_sid + 7, 5);
            // sub-authorities: 21, 0, 0, 0, 500 (admin-like)
            emu.maps.write_dword(p_sid + 8, 21);
            emu.maps.write_dword(p_sid + 12, 0);
            emu.maps.write_dword(p_sid + 16, 0);
            emu.maps.write_dword(p_sid + 20, 0);
            emu.maps.write_dword(p_sid + 24, 500);
            emu.set_last_error(0);
            call.set_return(emu, 1);
        } else {
            emu.set_last_error(87); // ERROR_INVALID_PARAMETER
            call.set_return(emu, 0);
        }
    } else {
        emu.set_last_error(87);
        call.set_return(emu, 0);
    }
}

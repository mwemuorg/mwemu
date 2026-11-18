use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InitializeSid(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sid_ptr = api_try!(emu, call.arg(emu, 0));
    let id_authority_ptr = api_try!(emu, call.arg(emu, 1));
    let sub_count = api_try!(emu, call.arg_u8(emu, 2));
    if sid_ptr != 0 {
        emu.maps.write_byte(sid_ptr, 1);
        emu.maps.write_byte(sid_ptr + 1, sub_count);
        if id_authority_ptr != 0 {
            let auth_bytes = emu.maps.read_bytes(id_authority_ptr, 6).to_vec();
            emu.maps.write_bytes(sid_ptr + 2, &auth_bytes);
        }
    }
    log_red!(
        emu,
        "advapi32!InitializeSid sid:0x{:x} count:{}",
        sid_ptr,
        sub_count
    );
    call.set_return(emu, 1);
}

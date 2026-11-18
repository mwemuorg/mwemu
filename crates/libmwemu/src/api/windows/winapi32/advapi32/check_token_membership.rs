use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CheckTokenMembership(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let token = api_try!(emu, call.arg(emu, 0));
    let sid_to_check = api_try!(emu, call.arg(emu, 1));
    let is_member_ptr = api_try!(emu, call.arg(emu, 2));
    if is_member_ptr != 0 {
        emu.maps.write_dword(is_member_ptr, 1); // TRUE - user is member
    }
    log_red!(
        emu,
        "advapi32!CheckTokenMembership token:0x{:x} sid:0x{:x} => TRUE",
        token,
        sid_to_check
    );
    call.cleanup_args(emu, 3);
    call.set_return(emu, 1);
}

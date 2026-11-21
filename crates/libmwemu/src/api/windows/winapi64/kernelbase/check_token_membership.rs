use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CheckTokenMembership(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let token = api_try!(emu, call.arg(emu, 0));
    let sid_to_check = api_try!(emu, call.arg(emu, 1));
    let is_member = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "kernelbase!CheckTokenMembership token:0x{:x} sid:0x{:x}",
        token,
        sid_to_check
    );
    if is_member != 0 {
        emu.maps.write_dword(is_member, 1); // TRUE - is a member
    }
    call.set_return(emu, 1); // success
}

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CheckTokenMembership(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let token_handle = api_try!(emu, call.arg(emu, 0));
    let _sid_to_check = api_try!(emu, call.arg(emu, 1));
    let is_member_ptr = api_try!(emu, call.arg(emu, 2));
    if is_member_ptr != 0 {
        emu.maps.write_dword(is_member_ptr, 1);
    }
    log_red!(
        emu,
        "advapi32!CheckTokenMembership token:0x{:x} => member=TRUE",
        token_handle
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

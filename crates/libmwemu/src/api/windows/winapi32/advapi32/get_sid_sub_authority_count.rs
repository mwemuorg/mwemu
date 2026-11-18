use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetSidSubAuthorityCount(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sid_ptr = api_try!(emu, call.arg(emu, 0));
    // Returns pointer to the sub-authority count byte at offset 1
    let result = sid_ptr + 1;
    log_red!(
        emu,
        "advapi32!GetSidSubAuthorityCount sid:0x{:x} => 0x{:x}",
        sid_ptr,
        result
    );
    call.cleanup_args(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, result);
}

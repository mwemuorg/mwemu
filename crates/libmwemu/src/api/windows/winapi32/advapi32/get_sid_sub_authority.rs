use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetSidSubAuthority(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sid_ptr = api_try!(emu, call.arg(emu, 0));
    let index = api_try!(emu, call.arg(emu, 1));
    // Returns pointer to the sub-authority DWORD at offset 8 + index*4
    let result = sid_ptr + 8 + index * 4;
    log_red!(
        emu,
        "advapi32!GetSidSubAuthority sid:0x{:x} idx:{} => 0x{:x}",
        sid_ptr,
        index,
        result
    );
    call.cleanup_args(emu, 2);
    emu.set_last_error(0);
    call.set_return(emu, result);
}

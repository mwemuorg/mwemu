use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetHandleInformation(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let flags_ptr = api_try!(emu, call.arg(emu, 1));
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0); // no flags set
    }
    log_red!(
        emu,
        "kernelbase!GetHandleInformation h:0x{:x} flags_ptr:0x{:x}",
        handle,
        flags_ptr
    );
    call.set_return(emu, 1); // TRUE
}

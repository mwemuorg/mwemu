use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetProcessHeap(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let out_buf = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "kernelbase!GetProcessHeap {}:0x{:x} {}:0x{:x}",
        "handle",
        handle,
        "out_buf",
        out_buf
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}

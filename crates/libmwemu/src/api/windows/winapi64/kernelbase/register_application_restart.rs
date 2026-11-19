use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn RegisterApplicationRestart(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cmdline_ptr = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let cmdline = if cmdline_ptr != 0 && cmdline_ptr > 0xffff {
        emu.maps.read_wide_string(cmdline_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!RegisterApplicationRestart cmdline:'{}' flags:0x{:x}",
        cmdline,
        flags
    );
    emu.set_last_error(0);
    call.set_return(emu, 0); // S_OK
}

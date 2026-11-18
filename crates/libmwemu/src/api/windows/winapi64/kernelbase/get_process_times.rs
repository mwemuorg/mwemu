use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetProcessTimes(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let creation_ptr = api_try!(emu, call.arg(emu, 1));
    let exit_ptr = api_try!(emu, call.arg(emu, 2));
    let kernel_ptr = api_try!(emu, call.arg(emu, 3));
    let user_ptr = api_try!(emu, call.arg(emu, 4));
    if creation_ptr != 0 {
        emu.maps.write_qword(creation_ptr, 0x01D0_0000_0000_0000);
    }
    if exit_ptr != 0 {
        emu.maps.write_qword(exit_ptr, 0);
    }
    if kernel_ptr != 0 {
        emu.maps.write_qword(kernel_ptr, 1000000);
    }
    if user_ptr != 0 {
        emu.maps.write_qword(user_ptr, 5000000);
    }
    log_red!(emu, "kernelbase!GetProcessTimes h:0x{:x}", handle);
    emu.set_last_error(0);
    call.set_return(emu, 0x000A0000);
}

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn OpenProcessToken(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let process_handle = api_try!(emu, call.arg(emu, 0));
    let _desired_access = api_try!(emu, call.arg(emu, 1));
    let token_handle_ptr = api_try!(emu, call.arg(emu, 2));

    log_red!(emu, "kernel32!OpenProcessToken hndl:0x{:x}", process_handle);

    if token_handle_ptr != 0 {
        emu.maps.write_qword(
            token_handle_ptr,
            helper::handler_create(&format!("token://{}", process_handle)) as u64,
        );
    }

    call.set_return(emu, 1); // TRUE
}

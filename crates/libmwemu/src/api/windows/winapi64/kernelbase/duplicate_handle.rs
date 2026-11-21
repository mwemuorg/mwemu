use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn DuplicateHandle(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let src_process = api_try!(emu, call.arg(emu, 0));
    let src_handle = api_try!(emu, call.arg(emu, 1));
    let dst_process = api_try!(emu, call.arg(emu, 2));
    let dst_handle_ptr = api_try!(emu, call.arg(emu, 3));
    let hndl = helper::handler_create("dup_handle");
    if dst_handle_ptr != 0 {
        emu.maps.write_qword(dst_handle_ptr, hndl);
    }
    log_red!(
        emu,
        "kernelbase!DuplicateHandle src:0x{:x} => 0x{:x}",
        src_handle,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

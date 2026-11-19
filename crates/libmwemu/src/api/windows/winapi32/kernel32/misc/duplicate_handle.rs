use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn DuplicateHandle(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let src_handle = api_try!(emu, call.arg_u32(emu, 1));
    let target_handle_ptr = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    log_red!(
        emu,
        "kernel32!DuplicateHandle src: 0x{:x} target_ptr: 0x{:x}",
        src_handle,
        target_handle_ptr
    );
    emu.maps.write_dword(target_handle_ptr, src_handle);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 7);
}

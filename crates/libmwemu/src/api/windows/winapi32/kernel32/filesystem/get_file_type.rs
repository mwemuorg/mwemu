use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetFileType(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hndl = api_try!(emu, call.arg_u32(emu, 0));

    log_red!(emu, "kernel32!GetFileType 0x{:x}", hndl);

    call.cleanup_args(emu, 1);
    call.set_return(emu, 3);

    /*
     * FILE_TYPE_CHAR 0x0002
     * FILE_TYPE_DISK 0x0001
     * FILE_TYPE_PIPE 0x0003
     * FILE_TYPE_REMOTE 0x8000
     * FILE_TYPE_UNKNOWN 0x0000
     */
}

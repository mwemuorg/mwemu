use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn DeleteFileA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let filename_ptr = api_try!(emu, call.arg(emu, 0));

    let filename = emu.maps.read_string(filename_ptr);

    log_red!(
        emu,
        "kernel32!DeleteFileA `{}` (stub: not deleting)",
        filename
    );

    call.cleanup_args(emu, 1);

    call.set_return(emu, 1); // Success
}

pub fn DeleteFileW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let filename = emu
        .maps
        .read_wide_string(api_try!(emu, call.arg_u32(emu, 0)) as u64);
    log_red!(
        emu,
        "kernel32!DeleteFileW `{}` (stub: not deleting)",
        filename
    );
    call.set_return(emu, 1);
    call.cleanup_args(emu, 1);
}

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn OpenProcessToken(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hndl = api_try!(emu, call.arg_u32(emu, 0));
    let access = api_try!(emu, call.arg_u32(emu, 1));
    let ptr_token = api_try!(emu, call.arg(emu, 2));

    log_red!(emu, "kernel32!OpenProcessToken 0x{:x} {}", hndl, access);

    emu.maps.write_dword(
        ptr_token,
        helper::handler_create(&format!("token://{}", hndl)) as u32,
    );

    call.cleanup_args(emu, 3);
    call.set_return(emu, 1);
}

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateRemoteThread(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let proc_hndl = api_try!(emu, call.arg(emu, 0));
    let sec = api_try!(emu, call.arg(emu, 1));
    let stack_size = api_try!(emu, call.arg(emu, 2));
    let addr = api_try!(emu, call.arg(emu, 3));
    let param = api_try!(emu, call.arg_u32(emu, 4));
    let flags = api_try!(emu, call.arg_u32(emu, 5));
    let out_tid = api_try!(emu, call.arg(emu, 6));

    log_red!(
        emu,
        "kernel32!CreateRemoteThread hproc: 0x{:x} addr: 0x{:x}",
        proc_hndl,
        addr
    );

    emu.maps.write_dword(out_tid, 0x123);
    call.set_return(emu, helper::handler_create("tid://0x123"));
    call.cleanup_args(emu, 7);
}

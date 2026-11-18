use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreatePipe(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let read_pipe = api_try!(emu, call.arg(emu, 0));
    let write_pipe = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "kernel32!CreatePipe");
    if read_pipe != 0 {
        emu.maps
            .write_qword(read_pipe, helper::handler_create("pipe_read"));
    }
    if write_pipe != 0 {
        emu.maps
            .write_qword(write_pipe, helper::handler_create("pipe_write"));
    }
    call.set_return(emu, 1);
}

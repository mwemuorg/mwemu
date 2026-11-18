use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateNamedPipeA(emu: &mut emu::Emu) {
    create_named_pipe(emu, "CreateNamedPipeA", false);
}

pub fn CreateNamedPipeW(emu: &mut emu::Emu) {
    create_named_pipe(emu, "CreateNamedPipeW", true);
}

fn create_named_pipe(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name_ptr = api_try!(emu, call.arg(emu, 0));
    let open_mode = api_try!(emu, call.arg(emu, 0));
    let pipe_mode = api_try!(emu, call.arg(emu, 2));
    let instances = api_try!(emu, call.arg(emu, 3));
    let out_buff_sz = api_try!(emu, call.arg(emu, 4));
    let in_buff_sz = api_try!(emu, call.arg(emu, 5));
    let timeout = api_try!(emu, call.arg(emu, 6));
    let security = api_try!(emu, call.arg(emu, 7));
    let name = strings::read_text(emu, name_ptr, wide);
    if wide {
        log_red!(
            emu,
            "kernel32!CreateNamedPipeA  name:{} in: 0x{:x} out: 0x{:x}",
            name,
            in_buff_sz,
            out_buff_sz
        );
    } else {
        log_red!(
            emu,
            "kernel32!{api}  name:{} in: 0x{:x} out: 0x{:x}",
            name,
            in_buff_sz,
            out_buff_sz
        );
    }

    call.set_return(emu, helper::handler_create(&name));
}

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateNamedPipeA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let name_ptr = api_try!(emu, call.arg(emu, 0));
    let open_mode = api_try!(emu, call.arg_u32(emu, 1));
    let pipe_mode = api_try!(emu, call.arg_u32(emu, 2));
    let instances = api_try!(emu, call.arg_u32(emu, 3));
    let out_buff_sz = api_try!(emu, call.arg_u32(emu, 4));
    let in_buff_sz = api_try!(emu, call.arg_u32(emu, 5));
    let timeout = api_try!(emu, call.arg_u32(emu, 6));
    let security = api_try!(emu, call.arg_u32(emu, 7));

    let name = emu.maps.read_string(name_ptr);

    log_red!(
        emu,
        "kernel32!CreateNamedPipeA  name:{} in: 0x{:x} out: 0x{:x}",
        name,
        in_buff_sz,
        out_buff_sz
    );

    call.cleanup_args(emu, 8);

    call.set_return(emu, helper::handler_create(&name));
}

pub fn CreateNamedPipeW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 8);
    log_red!(emu, "kernel32!CreateNamedPipeW");
    call.set_return(emu, helper::handler_create("CreateNamedPipeW"));
}

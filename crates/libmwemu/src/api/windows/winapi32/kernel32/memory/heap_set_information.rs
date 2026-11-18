use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn HeapSetInformation(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hndl = api_try!(emu, call.arg_u32(emu, 0));
    let hinfocls = api_try!(emu, call.arg_u32(emu, 1));
    let hinfo = api_try!(emu, call.arg_u32(emu, 2));
    let hinfo_sz = api_try!(emu, call.arg_u32(emu, 3));

    log_red!(
        emu,
        "kernel32!HeapSetInformation hndl=0x{:x} class={} info=0x{:x} sz={}",
        hndl,
        hinfocls,
        hinfo,
        hinfo_sz
    );

    call.cleanup_args(emu, 4);
    call.set_return(emu, 1);
}

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetProcessWorkingSetSize(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hprocess = api_try!(emu, call.arg(emu, 0));
    let min_size = api_try!(emu, call.arg(emu, 1));
    let max_size = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "kernel32!SetProcessWorkingSetSize proc:0x{:x} min:0x{:x} max:0x{:x}",
        hprocess,
        min_size,
        max_size
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

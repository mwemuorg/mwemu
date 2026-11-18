use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SafeArrayGetVartype(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sa_ptr = api_try!(emu, call.arg(emu, 0));
    let index = api_try!(emu, call.arg(emu, 1));
    let out_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "oleaut32!SafeArrayGetVartype sa:0x{:x} idx:0x{:x}",
        sa_ptr,
        index
    );
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}

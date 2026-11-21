use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SafeArrayAccessData(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sa = api_try!(emu, call.arg(emu, 0));
    let data_ptr = api_try!(emu, call.arg(emu, 1));
    if data_ptr != 0 {
        emu.maps.write_qword(data_ptr, sa + 0x20);
    }
    log_red!(emu, "oleaut32!SafeArrayAccessData 0x{:x}", sa);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}

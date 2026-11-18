use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WPUGetProviderPathEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let provider_id_ptr = api_try!(emu, call.arg(emu, 0));
    let path_ptr = api_try!(emu, call.arg(emu, 1));
    let path_len_ptr = api_try!(emu, call.arg(emu, 2));
    let flags = api_try!(emu, call.arg(emu, 3));
    log_red!(emu, "ws2_32!WPUGetProviderPathEx flags:{}", flags);
    let out_ptr = api_try!(emu, call.arg(emu, 3));
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}

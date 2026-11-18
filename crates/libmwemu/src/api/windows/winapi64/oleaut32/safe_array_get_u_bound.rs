use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SafeArrayGetUBound(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let psa = api_try!(emu, call.arg(emu, 0));
    let ndim = api_try!(emu, call.arg_u32(emu, 1));
    let pubound = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "oleaut32!SafeArrayGetUBound psa: 0x{:x} dim: {}",
        psa,
        ndim
    );
    if psa != 0 && pubound != 0 {
        let count = emu.maps.read_dword(psa + 24).unwrap_or(0);
        let lbound = emu.maps.read_dword(psa + 28).unwrap_or(0);
        let ubound = if count > 0 {
            lbound + count - 1
        } else {
            lbound
        };
        emu.maps.write_dword(pubound, ubound);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0); // S_OK
}

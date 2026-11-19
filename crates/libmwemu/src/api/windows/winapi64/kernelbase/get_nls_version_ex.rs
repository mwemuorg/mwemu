use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetNLSVersionEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let func = api_try!(emu, call.arg_u32(emu, 0));
    let locale_ptr = api_try!(emu, call.arg(emu, 1));
    let info_ptr = api_try!(emu, call.arg(emu, 2));
    let locale = if locale_ptr != 0 && locale_ptr > 0xffff {
        emu.maps.read_wide_string(locale_ptr)
    } else {
        "en-US".to_string()
    };
    if info_ptr != 0 {
        emu.maps.write_dword(info_ptr, 32); // dwNLSVersionInfoSize
        emu.maps.write_dword(info_ptr + 4, 6); // dwNLSVersion
        emu.maps.write_dword(info_ptr + 8, 2); // dwDefinedVersion
        emu.maps.write_dword(info_ptr + 12, 0); // dwEffectiveId
    }
    log_red!(
        emu,
        "kernelbase!GetNLSVersionEx func:{} locale:'{}'",
        func,
        locale
    );
    call.set_return(emu, 1); // TRUE
}

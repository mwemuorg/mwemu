use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn IsNormalizedString(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let norm_form = api_try!(emu, call.arg(emu, 0));
    let str_ptr = api_try!(emu, call.arg(emu, 1));
    let s = if str_ptr != 0 {
        emu.maps.read_wide_string(str_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!IsNormalizedString form:{} '{}' => true",
        norm_form,
        s
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

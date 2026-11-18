use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn GetClassInfoExA(emu: &mut emu::Emu) {
    get_class_info_ex(emu, "GetClassInfoExA", false);
}

pub(super) fn GetClassInfoExW(emu: &mut emu::Emu) {
    get_class_info_ex(emu, "GetClassInfoExW", true);
}

fn get_class_info_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let arg3 = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "user32!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    if wide {
        emu.set_last_error(0);
    }

    call.set_return(emu, 0);
}

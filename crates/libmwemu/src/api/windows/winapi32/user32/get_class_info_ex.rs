use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn GetClassInfoExA(emu: &mut emu::Emu) {
    get_class_info_ex(emu, "GetClassInfoExA");
}

fn get_class_info_ex(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!{api}");
    call.set_return(emu, 0);
    call.cleanup_args(emu, 3);
}

pub(super) fn GetClassInfoExW(emu: &mut emu::Emu) {
    get_class_info_ex(emu, "GetClassInfoExW");
}

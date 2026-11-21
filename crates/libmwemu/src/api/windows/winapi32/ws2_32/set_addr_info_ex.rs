use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn SetAddrInfoExA(emu: &mut emu::Emu) {
    set_addr_info_ex(emu, "SetAddrInfoExA", false);
}

fn set_addr_info_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name_ptr = api_try!(emu, call.arg(emu, 0));
    let name = if name_ptr != 0 {
        strings::read_text(emu, name_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "ws2_32!{api} name:'{}'", name);
    emu.set_last_error(0);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 12);
}

pub fn SetAddrInfoExW(emu: &mut emu::Emu) {
    set_addr_info_ex(emu, "SetAddrInfoExW", true);
}

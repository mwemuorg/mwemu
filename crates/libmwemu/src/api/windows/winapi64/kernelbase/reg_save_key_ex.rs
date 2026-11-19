use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn RegSaveKeyExA(emu: &mut emu::Emu) {
    reg_save_key_ex(emu, "RegSaveKeyExA", false);
}

fn reg_save_key_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let file_ptr = api_try!(emu, call.arg(emu, 1));
    let _security = api_try!(emu, call.arg(emu, 2));
    let flags = api_try!(emu, call.arg(emu, 3));
    let filename = if file_ptr != 0 {
        strings::read_text(emu, file_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!{api} hkey:0x{:x} '{}' flags:0x{:x}",
        hkey,
        filename,
        flags
    );
    emu.set_last_error(0);
    call.set_return(emu, 0); // ERROR_SUCCESS
}

pub fn RegSaveKeyExW(emu: &mut emu::Emu) {
    reg_save_key_ex(emu, "RegSaveKeyExW", true);
}

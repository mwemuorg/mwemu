use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn InternetReadFileExA(emu: &mut emu::Emu) {
    internet_read_file_ex(emu, "InternetReadFileExA");
}

pub(super) fn InternetReadFileExW(emu: &mut emu::Emu) {
    internet_read_file_ex(emu, "InternetReadFileExW");
}

fn internet_read_file_ex(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let file_hndl = api_try!(emu, call.arg(emu, 0));
    let buff_ptr = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "wininet!{api} buff: 0x{:x}", buff_ptr);
    if !helper::handler_exist(file_hndl) {
        log::trace!("\tinvalid handle.");
    }
    call.set_return(emu, 1);
}

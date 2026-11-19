use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn InternetSetOptionA(emu: &mut emu::Emu) {
    internet_set_option(emu, "InternetSetOptionA", false);
}

fn internet_set_option(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let inet_hndl = api_try!(emu, call.arg(emu, 0));
    let option = api_try!(emu, call.arg(emu, 1));
    let buffer = api_try!(emu, call.arg(emu, 2));
    let len = api_try!(emu, call.arg(emu, 3));

    let mut buffer_content = "".to_string();
    if buffer != 0 {
        buffer_content = emu.maps.read_string_of_bytes(buffer, len as usize);
    }
    let sbuff = strings::read_text(emu, buffer, wide);

    log_red!(
        emu,
        "wininet!{api} option: 0x{:x} buff: {{{}}} {}",
        option,
        buffer_content,
        sbuff
    );

    if !helper::handler_exist(inet_hndl) {
        log::trace!("\tinvalid handle.");
    }

    call.set_return(emu, 1); // true
}

pub(super) fn InternetSetOptionW(emu: &mut emu::Emu) {
    internet_set_option(emu, "InternetSetOptionW", true);
}

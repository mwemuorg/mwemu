use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn SetDlgItemTextA(emu: &mut emu::Emu) {
    set_dlg_item_text(emu, "SetDlgItemTextA", false);
}

fn set_dlg_item_text(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hdlg = api_try!(emu, call.arg(emu, 0));
    let id = api_try!(emu, call.arg_u32(emu, 1));
    let text_ptr = api_try!(emu, call.arg(emu, 2));
    let text = if text_ptr != 0 {
        strings::read_text(emu, text_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "user32!{api} id:{} '{}'", id, text);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn SetDlgItemTextW(emu: &mut emu::Emu) {
    set_dlg_item_text(emu, "SetDlgItemTextW", true);
}

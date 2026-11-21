use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetDlgItemTextA(emu: &mut emu::Emu) {
    get_dlg_item_text(emu, "GetDlgItemTextA", false);
}

pub fn GetDlgItemTextW(emu: &mut emu::Emu) {
    get_dlg_item_text(emu, "GetDlgItemTextW", true);
}

fn get_dlg_item_text(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _hdlg = api_try!(emu, call.arg(emu, 0));
    let _id = api_try!(emu, call.arg_u32(emu, 1));
    let buf = api_try!(emu, call.arg(emu, 2));
    let max_count = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(emu, "user32!{api} id:{} buf:0x{:x}", _id, buf);
    if wide {
        if buf != 0 && max_count > 0 {
            emu.maps.write_word(buf, 0);
        }
    } else {
        if buf != 0 && max_count > 0 {
            emu.maps.write_byte(buf, 0);
        }
    }

    call.set_return(emu, 0);
}

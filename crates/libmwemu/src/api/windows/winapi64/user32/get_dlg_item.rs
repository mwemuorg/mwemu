use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetDlgItem(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdlg = api_try!(emu, call.arg(emu, 0));
    let id = api_try!(emu, call.arg_u32(emu, 1));
    let hwnd = helper::handler_create("dlg_item");
    log_red!(
        emu,
        "user32!GetDlgItem dlg:0x{:x} id:{} => 0x{:x}",
        hdlg,
        id,
        hwnd
    );
    emu.set_last_error(0);
    call.set_return(emu, hwnd);
}

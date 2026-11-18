use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetDlgItemInt(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdlg = api_try!(emu, call.arg(emu, 0));
    let item_id = api_try!(emu, call.arg(emu, 1));
    let hndl = helper::handler_create(&format!("dlgitem_{}", item_id));
    log_red!(
        emu,
        "user32!GetDlgItemInt dlg:0x{:x} id:{} => 0x{:x}",
        hdlg,
        item_id,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}

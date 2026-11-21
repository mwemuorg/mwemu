use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateDialogIndirectParamA(emu: &mut emu::Emu) {
    create_dialog_indirect_param(emu, "CreateDialogIndirectParamA");
}

fn create_dialog_indirect_param(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let hinstance = api_try!(emu, call.arg(emu, 0));
    let template_ptr = api_try!(emu, call.arg(emu, 1));
    let hwnd_parent = api_try!(emu, call.arg(emu, 2));
    let dlg_proc = api_try!(emu, call.arg(emu, 3));
    let init_param = api_try!(emu, call.arg(emu, 4));
    log_red!(
        emu,
        "user32!{api} hinst:0x{:x} parent:0x{:x} proc:0x{:x}",
        hinstance,
        hwnd_parent,
        dlg_proc
    );
    let hndl = helper::handler_create(&format!("user32://{api}"));
    call.set_return(emu, hndl);
}

pub fn CreateDialogIndirectParamW(emu: &mut emu::Emu) {
    create_dialog_indirect_param(emu, "CreateDialogIndirectParamW");
}

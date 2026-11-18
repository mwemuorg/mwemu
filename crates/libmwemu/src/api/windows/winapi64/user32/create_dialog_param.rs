use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateDialogParamA(emu: &mut emu::Emu) {
    create_dialog_param(emu, "CreateDialogParamA", false);
}

fn create_dialog_param(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hinstance = api_try!(emu, call.arg(emu, 0));
    let template_name = api_try!(emu, call.arg(emu, 1));
    let hwnd_parent = api_try!(emu, call.arg(emu, 2));
    let dlg_proc = api_try!(emu, call.arg(emu, 3));
    let init_param = api_try!(emu, call.arg(emu, 4));
    let name = if template_name > 0xffff {
        strings::read_text(emu, template_name, wide)
    } else {
        format!("#{}", template_name)
    };
    log_red!(
        emu,
        "user32!{api} hinst:0x{:x} tmpl:'{}' parent:0x{:x} proc:0x{:x}",
        hinstance,
        name,
        hwnd_parent,
        dlg_proc
    );
    let uri = format!("user32://{api}/{}", name);
    let hndl = helper::handler_create(&uri);
    call.set_return(emu, hndl);
}

pub fn CreateDialogParamW(emu: &mut emu::Emu) {
    create_dialog_param(emu, "CreateDialogParamW", true);
}

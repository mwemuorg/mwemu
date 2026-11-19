use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn DialogBoxParamA(emu: &mut emu::Emu) {
    dialog_box_param(emu, "DialogBoxParamA", false);
}

fn dialog_box_param(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hinstance = api_try!(emu, call.arg(emu, 0));
    let template_ptr = api_try!(emu, call.arg(emu, 1));
    let hwnd_parent = api_try!(emu, call.arg(emu, 2));
    let dlg_proc = api_try!(emu, call.arg(emu, 3));
    let init_param = api_try!(emu, call.arg(emu, 4));
    let template_name = if template_ptr > 0xffff {
        strings::read_text(emu, template_ptr, wide)
    } else {
        format!("#{}", template_ptr)
    };
    log_red!(
        emu,
        "user32!{api} '{}' proc:0x{:x} param:0x{:x}",
        template_name,
        dlg_proc,
        init_param
    );
    emu.set_last_error(0);
    call.set_return(emu, 1); // IDOK
}

pub fn DialogBoxParamW(emu: &mut emu::Emu) {
    dialog_box_param(emu, "DialogBoxParamW", true);
}

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateDialogIndirectParamA(emu: &mut emu::Emu) {
    create_dialog_indirect_param(emu, "CreateDialogIndirectParamA", false);
}

pub fn CreateDialogIndirectParamW(emu: &mut emu::Emu) {
    create_dialog_indirect_param(emu, "CreateDialogIndirectParamW", true);
}

fn create_dialog_indirect_param(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let a0 = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let a1 = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    log_red!(emu, "user32!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
    if wide {
        call.set_return(emu, helper::handler_create("createdialogindirectparamw"));
    } else {
        call.set_return(emu, helper::handler_create("createdialogindirectparama"));
    }

    call.cleanup_args(emu, 5);
}

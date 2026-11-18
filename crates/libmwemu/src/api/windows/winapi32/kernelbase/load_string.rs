use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::trace;

pub(super) fn LoadStringA(emu: &mut Emu) {
    load(emu, "LoadStringA");
}

pub(super) fn LoadStringW(emu: &mut Emu) {
    load(emu, "LoadStringW");
}

fn load(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    trace::api(
        emu,
        &format!("kernelbase!{api}"),
        "module id buffer capacity",
        true,
        |emu| {
            // Resource string tables are not modelled here, as in user32's handler.
            call.set_return(emu, 0);
            call.cleanup_args(emu, 4);
        },
    );
}

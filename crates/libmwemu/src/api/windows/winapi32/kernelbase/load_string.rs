use crate::emu::Emu;
use crate::winapi::common::{memory::Call, trace};

pub(super) fn LoadStringA(emu: &mut Emu) {
    load(emu, "LoadStringA");
}

pub(super) fn LoadStringW(emu: &mut Emu) {
    load(emu, "LoadStringW");
}

fn load(emu: &mut Emu, api: &str) {
    trace::api(
        emu,
        &format!("kernelbase!{api}"),
        "module id buffer capacity",
        true,
        |emu| {
            // Resource string tables are not modelled here, as in user32's handler.
            Call::Api.finish(emu, 0, 4);
        },
    );
}

use super::unregister_class;
use crate::emu;

pub(super) fn UnregisterClassA(emu: &mut emu::Emu) {
    unregister_class(emu, "UnregisterClassA", false);
}

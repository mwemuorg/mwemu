use crate::emu;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().get_eax() as u32 {
        17 => super::trace_syscall32(emu, "break"),
        _ => return false,
    }

    true
}

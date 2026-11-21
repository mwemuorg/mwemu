use crate::emu;
use crate::syscall::linux::unsupported;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().get_eax() as u32 {
        17 => unsupported(emu, "break"),
        _ => return false,
    }

    true
}

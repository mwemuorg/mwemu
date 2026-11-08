use super::AllocKind;
use super::classify;
use crate::emu;

pub(super) fn crt_msize(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    let size = match classify(emu, ptr) {
        AllocKind::Arena { size } | AllocKind::Map { size, .. } => size as u64,
        AllocKind::Invalid => 0xFFFF_FFFF_FFFF_FFFFu64,
    };
    log_red!(emu, "wincrt!_msize 0x{:x} => {}", ptr, size);
    emu.regs_mut().rax = size;
}

use super::AllocKind;
use super::classify;
use crate::emu;

pub fn _msize(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    let size = match classify(emu, ptr) {
        AllocKind::Arena { size } | AllocKind::Map { size, .. } => size as u64,
        AllocKind::Invalid => u64::MAX,
    };
    log_red!(emu, "msvcrt!_msize 0x{:x} => {}", ptr, size);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = size;
}

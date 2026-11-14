use super::EINVAL;
use super::errno_cell;
use crate::emu;

pub(super) fn _get_errno(emu: &mut emu::Emu) {
    let out = emu.regs().rcx;
    if out == 0 {
        emu.regs_mut().rax = EINVAL as u64;
        return;
    }
    let cell = errno_cell(emu);
    let value = emu.maps.read_dword(cell).unwrap_or(0);
    emu.maps.write_dword(out, value);
    emu.regs_mut().rax = 0;
}

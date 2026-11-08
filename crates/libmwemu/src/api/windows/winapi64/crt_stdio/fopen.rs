use super::open;
use super::read_text;
use super::set_errno;
use crate::emu;

pub(super) fn fopen(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path = read_text(emu, emu.regs().rcx, wide);
    let mode = read_text(emu, emu.regs().rdx, wide);
    let result = open(emu, &path, &mode);
    log_red!(emu, "wincrt!{} '{}' '{}' = {:x?}", api, path, mode, result);
    match result {
        Ok(stream) => emu.regs_mut().rax = stream,
        Err(errno) => {
            set_errno(emu, errno);
            emu.regs_mut().rax = 0;
        }
    }
}

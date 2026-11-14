use super::EINVAL;
use super::open;
use super::read_text;
use super::set_errno;
use crate::emu;

pub(super) fn fopen_s(emu: &mut emu::Emu, api: &str, wide: bool) {
    let out = emu.regs().rcx;
    let path_ptr = emu.regs().rdx;
    let mode_ptr = emu.regs().r8;
    if out == 0 || path_ptr == 0 || mode_ptr == 0 {
        set_errno(emu, EINVAL);
        emu.regs_mut().rax = EINVAL as u64;
        return;
    }
    let path = read_text(emu, path_ptr, wide);
    let mode = read_text(emu, mode_ptr, wide);
    let result = open(emu, &path, &mode);
    log_red!(emu, "wincrt!{} '{}' '{}' = {:x?}", api, path, mode, result);
    match result {
        Ok(stream) => {
            emu.maps.write_qword(out, stream);
            emu.regs_mut().rax = 0;
        }
        Err(errno) => {
            emu.maps.write_qword(out, 0);
            set_errno(emu, errno);
            emu.regs_mut().rax = errno as u64;
        }
    }
}

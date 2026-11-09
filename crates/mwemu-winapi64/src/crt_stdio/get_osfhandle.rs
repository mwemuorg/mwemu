use super::EBADF;
use super::INVALID_HANDLE_VALUE;
use super::fd_handle;
use super::set_errno;
use crate::emu;

pub(super) fn get_osfhandle(emu: &mut emu::Emu) {
    let fd = emu.regs().rcx as i32;
    let handle = fd_handle(fd);
    log_red!(emu, "wincrt!_get_osfhandle fd {} = {:?}", fd, handle);
    match handle {
        Some(handle) => emu.regs_mut().rax = handle as u64,
        None => {
            set_errno(emu, EBADF);
            emu.regs_mut().rax = INVALID_HANDLE_VALUE;
        }
    }
}

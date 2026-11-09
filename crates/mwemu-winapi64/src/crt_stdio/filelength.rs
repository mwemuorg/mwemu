use super::EBADF;
use super::EINVAL;
use super::EOF;
use super::fd_handle;
use super::set_errno;
use crate::emu;

pub(super) fn filelength(emu: &mut emu::Emu, api: &str, long: bool) {
    let fd = emu.regs().rcx as i32;
    let len = fd_handle(fd).and_then(|handle| {
        emu.handle_management
            .get_mut_file_handle(handle)
            .and_then(|fh| fh.length().ok())
    });
    log_red!(emu, "wincrt!{} fd {} = {:?}", api, fd, len);
    match len {
        Some(len) if long && len > i32::MAX as u64 => {
            set_errno(emu, EINVAL);
            emu.regs_mut().rax = EOF;
        }
        Some(len) => emu.regs_mut().rax = len,
        None => {
            set_errno(emu, EBADF);
            emu.regs_mut().rax = if long { EOF } else { u64::MAX };
        }
    }
}

use super::EINVAL;
use super::EOF;
use super::set_errno;
use super::with_stream;
use crate::emu;

pub(super) fn fileno(emu: &mut emu::Emu) {
    let file = emu.regs().rcx;
    let fd = with_stream(file, |s| s.fd);
    log_red!(emu, "wincrt!_fileno 0x{:x} = {:?}", file, fd);
    match fd {
        Some(fd) => emu.regs_mut().rax = fd as u64,
        None => {
            set_errno(emu, EINVAL);
            emu.regs_mut().rax = EOF;
        }
    }
}

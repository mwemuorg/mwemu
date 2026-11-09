use super::super::wincrt::release;
use super::EINVAL;
use super::EOF;
use super::STATE;
use super::set_errno;
use crate::emu;

pub(super) fn fclose(emu: &mut emu::Emu) {
    let file = emu.regs().rcx;
    let removed = {
        let mut state = STATE.lock().unwrap();
        match state.streams.iter().position(|s| s.file == file) {
            Some(idx) => {
                let fd = state.streams.remove(idx).fd;
                let d = state.descriptors.iter().position(|d| d.fd == fd);
                d.map(|d| state.descriptors.remove(d).handle)
            }
            None => None,
        }
    };
    log_red!(emu, "wincrt!fclose 0x{:x} ok: {}", file, removed.is_some());
    match removed {
        Some(handle) => {
            emu.handle_management.remove_file_handle(handle);
            release(emu, file);
            emu.regs_mut().rax = 0;
        }
        None => {
            set_errno(emu, EINVAL);
            emu.regs_mut().rax = EOF;
        }
    }
}

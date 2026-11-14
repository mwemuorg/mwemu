use super::EINVAL;
use super::EOF;
use super::set_errno;
use super::stream_handle;
use crate::emu;

pub(super) fn ftell(emu: &mut emu::Emu, api: &str, long: bool) {
    let file = emu.regs().rcx;
    let pos = stream_handle(file).and_then(|handle| {
        emu.handle_management
            .get_mut_file_handle(handle)
            .map(|fh| fh.get_position())
    });
    log_red!(emu, "wincrt!{} 0x{:x} = {:?}", api, file, pos);
    match pos {
        Some(pos) if long && pos > i32::MAX as u64 => {
            set_errno(emu, EINVAL);
            emu.regs_mut().rax = EOF;
        }
        Some(pos) => emu.regs_mut().rax = pos,
        None => {
            set_errno(emu, EINVAL);
            emu.regs_mut().rax = if long { EOF } else { u64::MAX };
        }
    }
}

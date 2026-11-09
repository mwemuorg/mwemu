use super::EINVAL;
use super::Stream;
use super::set_errno;
use super::with_stream;
use crate::emu;

pub(super) fn stream_flag(emu: &mut emu::Emu, api: &str, flag: fn(&Stream) -> bool) {
    let file = emu.regs().rcx;
    let set = with_stream(file, |s| flag(s));
    log_red!(emu, "wincrt!{} 0x{:x} = {:?}", api, file, set);
    if set.is_none() {
        set_errno(emu, EINVAL);
    }
    emu.regs_mut().rax = set.unwrap_or(false) as u64;
}

use super::EINVAL;
use super::EOF;
use super::SEEK_CUR;
use super::SEEK_END;
use super::SEEK_SET;
use super::set_errno;
use super::stream_handle;
use super::with_stream;
use crate::emu;
use std::io::SeekFrom;

pub(super) fn fseek(emu: &mut emu::Emu, api: &str, offset: i64) {
    let file = emu.regs().rcx;
    let origin = emu.regs().r8 as u32 as u64;
    let target = match origin {
        SEEK_SET if offset >= 0 => Some(SeekFrom::Start(offset as u64)),
        SEEK_CUR => Some(SeekFrom::Current(offset)),
        SEEK_END => Some(SeekFrom::End(offset)),
        _ => None,
    };
    let pos = match (stream_handle(file), target) {
        (Some(handle), Some(target)) => emu
            .handle_management
            .get_mut_file_handle(handle)
            .and_then(|fh| fh.seek(target).ok()),
        _ => None,
    };
    log_red!(
        emu,
        "wincrt!{} 0x{:x} {} {} = {:?}",
        api,
        file,
        offset,
        origin,
        pos
    );
    if pos.is_some() {
        with_stream(file, |s| s.eof = false);
        emu.regs_mut().rax = 0;
    } else {
        set_errno(emu, EINVAL);
        emu.regs_mut().rax = EOF;
    }
}

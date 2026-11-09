use super::stream_handle;
use super::with_stream;
use crate::emu;
use std::io::SeekFrom;

pub(super) fn rewind(emu: &mut emu::Emu) {
    let file = emu.regs().rcx;
    log_red!(emu, "wincrt!rewind 0x{:x}", file);
    if let Some(handle) = stream_handle(file)
        && let Some(fh) = emu.handle_management.get_mut_file_handle(handle)
    {
        let _ = fh.seek(SeekFrom::Start(0));
    }
    with_stream(file, |s| {
        s.eof = false;
        s.error = false;
    });
}

use super::EINVAL;
use super::EOF;
use super::set_errno;
use super::stream_handle;
use super::with_stream;
use crate::emu;

pub(super) fn fgetc(emu: &mut emu::Emu, api: &str) {
    let file = emu.regs().rcx;
    let Some(handle) = stream_handle(file) else {
        set_errno(emu, EINVAL);
        emu.regs_mut().rax = EOF;
        return;
    };
    let mut byte = [0u8; 1];
    let read = emu
        .handle_management
        .get_mut_file_handle(handle)
        .and_then(|fh| fh.read(&mut byte).ok());
    log_red!(
        emu,
        "wincrt!{} 0x{:x} = {:?}",
        api,
        file,
        read.map(|_| byte[0])
    );
    match read {
        Some(1) => emu.regs_mut().rax = byte[0] as u64,
        _ => {
            with_stream(file, |s| s.eof = true);
            emu.regs_mut().rax = EOF;
        }
    }
}

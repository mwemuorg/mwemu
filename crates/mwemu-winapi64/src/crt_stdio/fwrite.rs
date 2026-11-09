use super::EINVAL;
use super::set_errno;
use super::stream_handle;
use super::with_stream;
use crate::emu;
use std::io::SeekFrom;

pub(super) fn fwrite(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let size = emu.regs().rdx;
    let count = emu.regs().r8;
    let file = emu.regs().r9;
    let total = size.saturating_mul(count) as usize;
    let data = emu.maps.try_read_bytes(buf, total).map(|b| b.to_vec());
    let append = with_stream(file, |s| s.append).unwrap_or(false);
    let written = match (stream_handle(file), data) {
        (Some(handle), Some(data)) => {
            emu.handle_management
                .get_mut_file_handle(handle)
                .and_then(|fh| {
                    if append {
                        fh.seek(SeekFrom::End(0)).ok()?;
                    }
                    fh.write(&data).ok()
                })
        }
        _ => None,
    };
    log_red!(
        emu,
        "wincrt!fwrite buf:0x{:x} size:{} count:{} fp:0x{:x} = {:?}",
        buf,
        size,
        count,
        file,
        written
    );
    match written {
        Some(n) if size != 0 => emu.regs_mut().rax = n as u64 / size,
        Some(_) => emu.regs_mut().rax = 0,
        None => {
            with_stream(file, |s| s.error = true);
            set_errno(emu, EINVAL);
            emu.regs_mut().rax = 0;
        }
    }
}

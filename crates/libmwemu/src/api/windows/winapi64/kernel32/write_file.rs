use crate::emu;
use crate::winapi::helper;

pub fn WriteFile(emu: &mut emu::Emu) {
    let file_hndl = emu.regs().rcx;
    let buff = emu.regs().rdx;
    let size = emu.regs().r8;
    let bytes_written = emu.regs().r9;
    let overlapped = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    emu.count_write += 1;

    emu.maps.write_qword(bytes_written, size);

    log_red!(
        emu,
        "kernel32!WriteFile hndl: 0x{:x} buff: 0x{:x} sz: {}",
        file_hndl,
        buff,
        size
    );

    let is_std = matches!(file_hndl, 0xfffffffffffffff4..=0xfffffffffffffff6);

    if is_std && size > 0 && size < 4096 {
        let bytes = emu.maps.read_bytes(buff, size as usize);
        let text = String::from_utf8_lossy(bytes);
        let stream = match file_hndl {
            0xfffffffffffffff6 => "stdin",
            0xfffffffffffffff5 => "stdout",
            0xfffffffffffffff4 => "stderr",
            _ => "std?",
        };
        log_red!(emu, "\t{}> {}", stream, text);
    } else if !helper::handler_exist(file_hndl) && !is_std {
        log::trace!("\tinvalid handle.");
    }

    emu.regs_mut().rax = 1;
}

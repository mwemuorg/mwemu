use super::COUNT_RECEIVE;
use crate::emu;
use crate::winapi::helper;

pub(super) fn InternetReadFile(emu: &mut emu::Emu) {
    let file_hndl = emu.regs().rcx;
    let buff_ptr = emu.regs().rdx;
    let bytes_to_read = emu.regs().r8;
    let bytes_read_ptr = emu.regs().r9;

    log_red!(
        emu,
        "wininet!InternetReadFile sz: {} buff: 0x{:x}",
        bytes_to_read,
        buff_ptr
    );

    if !helper::handler_exist(file_hndl) {
        log::trace!("\tinvalid handle.");
    }

    if emu.cfg.endpoint {
        /*
        let buff = endpoint::http_read_data();
        emu.maps.write_buffer(buff_ptr, &buff);
        emu.maps.write_dword(bytes_read_ptr, buff.len() as u32);
        */
    } else {
        let mut count = COUNT_RECEIVE.lock().unwrap();
        *count += 1;

        if *count < 3 {
            emu.maps.write_spaced_bytes(buff_ptr, "90 90 90 90");
            emu.maps.write_dword(bytes_read_ptr, bytes_to_read as u32);
        } else {
            emu.maps.write_dword(bytes_read_ptr, 0);
        }
    }

    emu.regs_mut().rax = 1; // true
}

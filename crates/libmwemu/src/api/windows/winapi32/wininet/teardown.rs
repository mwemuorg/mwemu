use crate::emu;
use crate::winapi::helper;

use super::{pop_stack32, read_stack_dword};

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref COUNT_RECEIVE: Mutex<u32> = Mutex::new(0);
}

pub(super) fn internet_read_file(emu: &mut emu::Emu) {
    let file_hndl =
        read_stack_dword(emu, 0, "wininet!InternetReadFile cannot read file_hndl") as u64;
    let buff_ptr = read_stack_dword(emu, 4, "wininet!InternetReadFile cannot read buff_ptr") as u64;
    let bytes_to_read =
        read_stack_dword(emu, 8, "wininet!InternetReadFile cannot read bytes_to_read") as u64;
    let bytes_read_ptr =
        read_stack_dword(emu, 12, "wininet!InternetReadFile cannot read bytes_read") as u64;

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

    pop_stack32(emu, 4);

    emu.regs_mut().rax = 1;
}

pub(super) fn internet_error_dlg(emu: &mut emu::Emu) {
    let err = read_stack_dword(emu, 8, "wininet!InternetErrorDlg cannot read error");

    log_red!(emu, "wininet!InternetErrorDlg err: {}", err);

    pop_stack32(emu, 5);
    emu.regs_mut().rax = 0;
}

pub(super) fn internet_close_handle(emu: &mut emu::Emu) {
    let handle = read_stack_dword(emu, 0, "wininet!InternetCloseHandle cannot read handle") as u64;

    log_red!(emu, "wininet!InternetCloseHandle handle: {:x}", handle);

    helper::handler_close(handle);
    pop_stack32(emu, 1);
    emu.regs_mut().rax = 1;
}

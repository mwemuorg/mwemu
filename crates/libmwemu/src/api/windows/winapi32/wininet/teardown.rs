use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref COUNT_RECEIVE: Mutex<u32> = Mutex::new(0);
}

pub(super) fn internet_read_file(emu: &mut emu::Emu) {
    let file_hndl = WinApiAbi::arg(emu, 0);
    let buff_ptr = WinApiAbi::arg(emu, 1);
    let bytes_to_read = WinApiAbi::arg(emu, 2);
    let bytes_read_ptr = WinApiAbi::arg(emu, 3);

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

    WinApiAbi::stdcall_ret(emu, 4);

    emu.regs_mut().rax = 1;
}

pub(super) fn internet_error_dlg(emu: &mut emu::Emu) {
    let err = WinApiAbi::arg(emu, 2) as u32;

    log_red!(emu, "wininet!InternetErrorDlg err: {}", err);

    WinApiAbi::stdcall_ret(emu, 5);
    emu.regs_mut().rax = 0;
}

pub(super) fn internet_close_handle(emu: &mut emu::Emu) {
    let handle = WinApiAbi::arg(emu, 0);

    log_red!(emu, "wininet!InternetCloseHandle handle: {:x}", handle);

    helper::handler_close(handle);
    WinApiAbi::stdcall_ret(emu, 1);
    emu.regs_mut().rax = 1;
}

pub(super) fn internet_read_file_ex_a(emu: &mut emu::Emu) {
    let file_hndl = WinApiAbi::arg(emu, 0);
    let buffers_ptr = WinApiAbi::arg(emu, 1);
    log_red!(
        emu,
        "wininet!InternetReadFileExA buffers: 0x{:x}",
        buffers_ptr
    );
    if !helper::handler_exist(file_hndl) {
        log::trace!("\tinvalid handle.");
    }
    // INTERNET_BUFFERS.dwBufferLength is at offset 12 (after dwStructSize,
    // lpcszHeader, dwHeadersLength... layout differs, but reporting 0 bytes
    // available keeps the caller's read loop terminating cleanly).
    if buffers_ptr != 0 {
        emu.maps.write_dword(buffers_ptr + 12, 0);
    }
    WinApiAbi::stdcall_ret(emu, 4);
    emu.regs_mut().rax = 1;
}

pub(super) fn internet_read_file_ex_w(emu: &mut emu::Emu) {
    internet_read_file_ex_a(emu);
}

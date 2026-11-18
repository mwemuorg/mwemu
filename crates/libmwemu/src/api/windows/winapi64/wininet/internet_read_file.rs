use super::COUNT_RECEIVE;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn InternetReadFile(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let file_hndl = api_try!(emu, call.arg(emu, 0));
    let buff_ptr = api_try!(emu, call.arg(emu, 1));
    let bytes_to_read = api_try!(emu, call.arg(emu, 2));
    let bytes_read_ptr = api_try!(emu, call.arg(emu, 3));

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

    call.set_return(emu, 1); // true
}

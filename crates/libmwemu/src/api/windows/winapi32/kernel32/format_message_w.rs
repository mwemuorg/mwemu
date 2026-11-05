use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn FormatMessageW(emu: &mut emu::Emu) {
    let flags = arg(emu, 0);
    let _source = arg(emu, 1);
    let msg_id = arg(emu, 2);
    let _lang_id = arg(emu, 3);
    let lp_buffer = arg(emu, 4) as u64;
    let n_size = arg(emu, 5);

    log_red!(
        emu,
        "kernel32!FormatMessageW flags:0x{:x} msgId:{} buf:0x{:x}",
        flags,
        msg_id,
        lp_buffer
    );

    let msg = error_message_w(msg_id);

    const FORMAT_MESSAGE_ALLOCATE_BUFFER: u32 = 0x00000100;

    if flags & FORMAT_MESSAGE_ALLOCATE_BUFFER != 0 {
        let byte_len = (msg.len() + 1) * 2;
        let alloc = emu.maps.alloc(byte_len as u64).unwrap_or(0);
        if alloc != 0 {
            emu.maps.write_wide_string(alloc, &msg);
            if lp_buffer != 0 {
                emu.maps.write_dword(lp_buffer, alloc as u32);
            }
        }
        emu.regs_mut().rax = msg.len() as u64;
    } else if lp_buffer != 0 {
        emu.maps.write_wide_string(lp_buffer, &msg);
        emu.regs_mut().rax = msg.len() as u64;
    } else {
        emu.set_last_error(87);
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 7);
}

fn error_message_w(code: u32) -> String {
    match code {
        0 => "The operation completed successfully.\r\n".to_string(),
        2 => "The system cannot find the file specified.\r\n".to_string(),
        3 => "The system cannot find the path specified.\r\n".to_string(),
        5 => "Access is denied.\r\n".to_string(),
        6 => "The handle is invalid.\r\n".to_string(),
        87 => "The parameter is incorrect.\r\n".to_string(),
        126 => "The specified module could not be found.\r\n".to_string(),
        127 => "The specified procedure could not be found.\r\n".to_string(),
        183 => "Cannot create a file when that file already exists.\r\n".to_string(),
        259 => "No more data is available.\r\n".to_string(),
        _ => format!("Unknown error 0x{:08x}.\r\n", code),
    }
}

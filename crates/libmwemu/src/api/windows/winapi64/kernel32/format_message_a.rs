use crate::emu;
use crate::windows::constants::system::FORMAT_MESSAGE_ALLOCATE_BUFFER;

pub fn FormatMessageA(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx as u32;
    let _source = emu.regs().rdx;
    let msg_id = emu.regs().r8 as u32;
    let _lang_id = emu.regs().r9;
    let lp_buffer = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let n_size = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    log_red!(
        emu,
        "kernel32!FormatMessageA flags:0x{:x} msgId:{} buf:0x{:x} sz:{}",
        flags,
        msg_id,
        lp_buffer,
        n_size
    );

    let msg = error_message(msg_id);

    if flags & FORMAT_MESSAGE_ALLOCATE_BUFFER != 0 {
        // Caller passes a pointer-to-pointer; we allocate and write there
        let alloc = emu.maps.alloc(msg.len() as u64 + 1).unwrap_or(0);
        if alloc != 0 {
            emu.maps.write_string(alloc, &msg);
            if lp_buffer != 0 {
                emu.maps.write_qword(lp_buffer, alloc);
            }
        }
        emu.regs_mut().rax = msg.len() as u64;
    } else if lp_buffer != 0 {
        emu.maps.write_string(lp_buffer, &msg);
        emu.regs_mut().rax = msg.len() as u64;
    } else {
        emu.set_last_error(87);
        emu.regs_mut().rax = 0;
    }
}

fn error_message(code: u32) -> String {
    match code {
        0 => "The operation completed successfully.\r\n".to_string(),
        1 => "Incorrect function.\r\n".to_string(),
        2 => "The system cannot find the file specified.\r\n".to_string(),
        3 => "The system cannot find the path specified.\r\n".to_string(),
        4 => "The system cannot open the file.\r\n".to_string(),
        5 => "Access is denied.\r\n".to_string(),
        6 => "The handle is invalid.\r\n".to_string(),
        8 => "Not enough storage is available to process this command.\r\n".to_string(),
        14 => "Not enough storage is available to complete this operation.\r\n".to_string(),
        15 => "The system cannot find the drive specified.\r\n".to_string(),
        18 => "There are no more files.\r\n".to_string(),
        32 => "The process cannot access the file because it is being used by another process.\r\n"
    .to_string(),
        87 => "The parameter is incorrect.\r\n".to_string(),
        109 => "The pipe has been ended.\r\n".to_string(),
        111 => "All pipe instances are busy.\r\n".to_string(),
        120 => "This function is not supported on this system.\r\n".to_string(),
        122 => "The data area passed to a system call is too small.\r\n".to_string(),
        126 => "The specified module could not be found.\r\n".to_string(),
        127 => "The specified procedure could not be found.\r\n".to_string(),
        183 => "Cannot create a file when that file already exists.\r\n".to_string(),
        203 => "The system could not find the environment option that was entered.\r\n".to_string(),
        259 => "No more data is available.\r\n".to_string(),
        267 => "The directory name is invalid.\r\n".to_string(),
        534 => "Arithmetic result exceeded 32 bits.\r\n".to_string(),
        995 => "The I/O operation has been aborted because of either a thread exit or an application request.\r\n".to_string(),
        997 => "Overlapped I/O operation is in progress.\r\n".to_string(),
        998 => "Invalid access to memory location.\r\n".to_string(),
        1008 => "An attempt was made to reference a token that does not exist.\r\n".to_string(),
        1150 => "The specified program requires a newer version of Windows.\r\n".to_string(),
        1168 => "Element not found.\r\n".to_string(),
        _ => format!("Unknown error 0x{:08x}.\r\n", code),
    }
}

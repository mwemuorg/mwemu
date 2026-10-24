use crate::emu;
use crate::winapi::helper;

pub fn ReadFile(emu: &mut emu::Emu) {
    let file_hndl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let buff = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let size = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let bytes_read = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let overlapped = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    emu.count_read += 1;

    if size == 4 && emu.count_read == 1 {
        // probably reading the size
        emu.maps.write_dword(buff, 0x10);
    }

    if emu.count_read < 3 {
        // keep reading bytes
        emu.maps.write_dword(bytes_read, size);
        emu.maps.memset(buff, 0x90, size as usize);
        emu.regs_mut().rax = 1;
    } else {
        // try to force finishing reading and continue the malware logic
        emu.maps.write_dword(bytes_read, 0);
        emu.regs_mut().rax = 0;
    }

    //TODO: write some random bytes to the buffer
    //emu.maps.write_spaced_bytes(buff, "00 00 00 01".to_string());

    log_red!(
        emu,
        "kernel32!ReadFile hndl: 0x{:x} buff: 0x{:x} sz: {}",
        file_hndl,
        buff,
        size
    );

    if !helper::handler_exist(file_hndl) {
        log::trace!("\tinvalid handle.")
    }

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}

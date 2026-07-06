use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32::COUNT_WRITE;

pub fn WriteFile(emu: &mut emu::Emu) {
    let file_hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!WriteFile cannot read the file_hndl") as u64;
    let buff = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!WriteFile cannot read the buff") as u64;
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!WriteFile cannot read the size");
    let bytes_written = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!WriteFile cannot read the bytes_written") as u64;
    let overlapped = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("kernel32!WriteFile cannot read the overlapped");

    let mut count = COUNT_WRITE.lock().unwrap();
    *count += 1;

    emu.maps.write_dword(bytes_written, size);

    log_red!(
        emu,
        "kernel32!WriteFile hndl: 0x{:x} buff: 0x{:x} sz: {}",
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
    emu.regs_mut().rax = 1;
}

use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi64::kernel32::COUNT_WRITE;

pub fn WriteFile(emu: &mut emu::Emu) {
    let file_hndl = emu.regs().rcx;
    let buff = emu.regs().rdx;
    let size = emu.regs().r8;
    let bytes_written = emu.regs().r9;
    let overlapped = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!WriteFile cannot read the overlapped");

    let mut count = COUNT_WRITE.lock().unwrap();
    *count += 1;

    emu.maps.write_qword(bytes_written, size);

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

    emu.regs_mut().rax = 1;
}

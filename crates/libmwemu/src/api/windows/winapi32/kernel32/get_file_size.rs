use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetFileSize(emu: &mut emu::Emu) {
    let h_file = arg(emu, 0) as u64;
    let lp_file_size_high = arg(emu, 1) as u64;
    log_red!(
        emu,
        "** {} kernel32!GetFileSize {:x} {:x}",
        emu.pos,
        h_file,
        lp_file_size_high
    );

    let Some(file_handle) = emu.handle_management.get_mut_file_handle(h_file as u32) else {
        log::warn!("kernel32!GetFileSize: invalid file handle 0x{:x}", h_file);
        emu.regs_mut().set_eax(0xFFFFFFFF);
        return;
    };
    let file_size = file_handle.file_size;
    emu.regs_mut().set_eax(file_size);
    if lp_file_size_high != 0x0 {
        emu.maps
            .write_dword(lp_file_size_high, (file_size >> 32) as u32);
    }
    stdcall_ret(emu, 2);
}

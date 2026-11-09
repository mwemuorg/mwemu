use crate::emu;

pub fn GetFileSizeEx(emu: &mut emu::Emu) {
    let hfile = emu.regs().rcx;
    let size_ptr = emu.regs().rdx;
    let file_size: u64 = 4096;
    if size_ptr != 0 {
        emu.maps.write_qword(size_ptr, file_size);
    }
    log_red!(
        emu,
        "kernelbase!GetFileSizeEx h:0x{:x} => {}",
        hfile,
        file_size
    );
    emu.regs_mut().rax = 1; // TRUE
}

use crate::emu;

pub fn WriteProcessMemory(emu: &mut emu::Emu) {
    let proc_hndl = emu.regs().rcx;
    let addr = emu.regs().rdx;
    let buff = emu.regs().r8;
    let size = emu.regs().r9;
    let written_ptr = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!WriteProcessMemory cannot read written_ptr");

    log_red!(
        emu,
        "kernel32!WriteProcessMemory hproc: 0x{:x} from: 0x{:x } to: 0x{:x} sz: {}",
        proc_hndl,
        buff,
        addr,
        size
    );

    if emu.maps.memcpy(buff, addr, size as usize) {
        emu.regs_mut().rax = 1;
        log::trace!(
            "{}\twritten succesfully{}",
            emu.colors.light_red,
            emu.colors.nc
        );
        if written_ptr != 0 && !emu.maps.write_qword(written_ptr, size) {
            log::trace!("kernel32!WriteProcessMemory cannot write on written_ptr");
        }
    } else {
        emu.regs_mut().rax = 0;
        log::trace!(
            "{}\tcouldnt write all the bytes{}",
            emu.colors.light_red,
            emu.colors.nc
        );
        if written_ptr != 0 && !emu.maps.write_qword(written_ptr, 0) {
            log::trace!("kernel32!WriteProcessMemory cannot write on written_ptr");
        }
    }
}

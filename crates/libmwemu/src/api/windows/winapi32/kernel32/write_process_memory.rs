use crate::emu;

pub fn WriteProcessMemory(emu: &mut emu::Emu) {
    let proc_hndl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let addr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let buff = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let size = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let written_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

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
    } else {
        emu.regs_mut().rax = 0;
        log::trace!(
            "{}\tcouldnt write the bytes{}",
            emu.colors.light_red,
            emu.colors.nc
        );
    }

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}

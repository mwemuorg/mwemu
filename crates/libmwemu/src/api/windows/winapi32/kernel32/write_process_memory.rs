use crate::emu;

pub fn WriteProcessMemory(emu: &mut emu::Emu) {
    let proc_hndl =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("kernel32!WriteProcessMemory cannot read the proc handle") as u64;
    let addr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!WriteProcessMemory cannot read the address") as u64;
    let buff = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!WriteProcessMemory cannot read the buffer") as u64;
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!WriteProcessMemory cannot read the size") as u64;
    let written_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("kernel32!WriteProcessMemory cannot read the ptr of num of written bytes");

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

use crate::emu;
use crate::maps::mem64::Permission;

pub fn MapViewOfFile(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!MapViewOfFile cannot read the handle");
    let access = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!MapViewOfFile cannot read the acess");
    let off_high = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!MapViewOfFile cannot read the off_hight") as u64;
    let off_low = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!MapViewOfFile cannot read the off_low") as u64;
    let mut size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("kernel32!MapViewOfFile cannot read the size") as u64;

    let off: u64 = (off_high << 32) + off_low;

    /*if size > 1024 * 4 {
        size = 1024
    }*/
    if size < 1024 {
        size = 1024;
    }
    let addr = emu
        .maps
        .alloc(size)
        .expect("kernel32!MapViewOfFile cannot allocate");
    let mem = emu
        .maps
        .create_map("file_map", addr, size, Permission::READ_WRITE)
        .expect("kernel32!MapViewOfFile cannot create map");
    let loaded = mem.load_chunk(&emu.filename, off, size as usize);

    log_red!(
        emu,
        "kernel32!MapViewOfFile hndl: {} off: {} sz: {} ={}",
        hndl,
        off,
        size,
        addr
    );

    if off > 0 {
        log::trace!("the non-zero offset is not implemented for now");
    }

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = addr;
}

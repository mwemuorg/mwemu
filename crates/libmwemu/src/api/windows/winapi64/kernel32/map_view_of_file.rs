use crate::emu;
use crate::maps::mem64::Permission;

pub fn MapViewOfFile(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let access = emu.regs().rdx;
    let off_high = emu.regs().r8;
    let off_low = emu.regs().r9;
    let mut size = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!MapViewOfFile cannot read the size");

    let off: u64 = (off_high << 32) + off_low;

    if size > 1024 * 4 {
        size = 1024
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

    emu.regs_mut().rax = addr;
}

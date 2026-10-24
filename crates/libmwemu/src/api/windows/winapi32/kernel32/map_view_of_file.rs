use crate::emu;
use crate::maps::mem64::Permission;

pub fn MapViewOfFile(emu: &mut emu::Emu) {
    let hndl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let access = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let off_high = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let off_low = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let mut size = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

    let off: u64 = (off_high << 32) + off_low;

    /*if size > 1024 * 4 {
        size = 1024
    }*/
    if size < 1024 {
        size = 1024;
    }
    let addr = match emu.maps.alloc(size) {
        Some(a) => a,
        None => {
            log::error!("kernel32!MapViewOfFile cannot allocate (size={})", size);
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
            return;
        }
    };
    let mem = match emu
        .maps
        .create_map("file_map", addr, size, Permission::READ_WRITE)
    {
        Ok(m) => m,
        Err(e) => {
            log::error!("kernel32!MapViewOfFile cannot create map: {}", e);
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
            return;
        }
    };
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

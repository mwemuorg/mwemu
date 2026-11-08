use crate::emu;

pub fn CoTaskMemRealloc(emu: &mut emu::Emu) {
    let old_ptr = emu.regs().rcx;
    let new_size = emu.regs().rdx;
    log_red!(
        emu,
        "ole32!CoTaskMemRealloc old:0x{:x} size:{}",
        old_ptr,
        new_size
    );
    if new_size == 0 {
        if old_ptr != 0 {
            let name = format!("cotaskmem_{:x}", old_ptr);
            emu.maps.free(&name);
        }
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
    } else if let Some(base) = emu.maps.alloc(new_size) {
        let name = format!("cotaskmem_{:x}", base);
        emu.maps.create_map(
            &name,
            base,
            new_size,
            crate::maps::mem64::Permission::READ_WRITE,
        );
        if old_ptr != 0 {
            let old_name = format!("cotaskmem_{:x}", old_ptr);
            if let Some(old_map) = emu.maps.get_map_by_name(&old_name) {
                let old_size = old_map.size() as u64;
                let copy_size = std::cmp::min(old_size, new_size);
                for i in 0..copy_size {
                    let b = emu.maps.read_byte(old_ptr + i).unwrap_or(0);
                    emu.maps.write_byte(base + i, b);
                }
            }
            emu.maps.free(&old_name);
        }
        emu.set_last_error(0);
        emu.regs_mut().rax = base;
    } else {
        emu.regs_mut().rax = 0;
    }
}

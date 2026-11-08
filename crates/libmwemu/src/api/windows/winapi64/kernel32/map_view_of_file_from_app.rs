use crate::emu;
use crate::winapi::helper;

pub fn MapViewOfFileFromApp(emu: &mut emu::Emu) {
    let file_mapping = emu.regs().rcx;
    let desired_access = emu.regs().rdx;
    let offset = emu.regs().r8;
    let num_bytes = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!MapViewOfFileFromApp mapping:0x{:x} access:0x{:x} offset:0x{:x} bytes:{}",
        file_mapping,
        desired_access,
        offset,
        num_bytes
    );
    if helper::handler_exist(file_mapping) {
        let sz = if num_bytes > 0 {
            num_bytes as usize
        } else {
            0x1000
        };
        let addr = emu.maps.alloc(sz as u64).expect("mapview alloc");
        emu.maps
            .create_map(
                &format!("mapview_{:x}", addr),
                addr,
                sz as u64,
                crate::maps::mem64::Permission::READ_WRITE,
            )
            .expect("mapview map");
        emu.regs_mut().rax = addr;
    } else {
        emu.regs_mut().rax = 0;
    }
}

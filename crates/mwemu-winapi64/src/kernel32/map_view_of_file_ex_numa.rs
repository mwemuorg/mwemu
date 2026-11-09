use crate::emu;
use crate::winapi::helper;

pub fn MapViewOfFileExNuma(emu: &mut emu::Emu) {
    let file_mapping = emu.regs().rcx;
    let desired_access = emu.regs().rdx;
    let offset_high = emu.regs().r8;
    let offset_low = emu.regs().r9;
    let num_bytes = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let base_addr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let numa_node = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as u32;
    log_red!(
        emu,
        "kernel32!MapViewOfFileExNuma mapping:0x{:x} access:0x{:x} offset:0x{:x}:{:x} bytes:{} base:0x{:x} numa:{}",
        file_mapping,
        desired_access,
        offset_high,
        offset_low,
        num_bytes,
        base_addr,
        numa_node
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

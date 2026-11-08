use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn MapViewOfFileExNuma(emu: &mut emu::Emu) {
    let file_mapping = arg(emu, 0) as u64;
    let desired_access = arg(emu, 1) as u64;
    let offset_high = arg(emu, 2) as u64;
    let offset_low = arg(emu, 3) as u64;
    let num_bytes = arg(emu, 4) as u64;
    let base_addr = arg(emu, 5) as u64;
    let numa_node = arg(emu, 6);
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
                Permission::READ_WRITE,
            )
            .expect("mapview map");
        emu.regs_mut().rax = addr;
    } else {
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 7);
}

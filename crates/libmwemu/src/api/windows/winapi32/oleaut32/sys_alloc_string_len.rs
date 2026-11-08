use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn SysAllocStringLen(emu: &mut emu::Emu) {
    let str_ptr = arg(emu, 0) as u64;
    let mut size = arg(emu, 1) as u64;

    if size == 0xffffffff {
        size = 1024;
    }
    let alloc_size = size * 2 + 2 + 8;

    let base = match emu.maps.alloc(alloc_size) {
        Some(a) => a,
        None => {
            log::error!("oleaut32!SysAllocStringLen out of memory");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
            return;
        }
    };
    let name = format!("alloc_{:x}", base + 8);
    emu.maps
        .create_map(&name, base, alloc_size, Permission::READ_WRITE);
    emu.maps.write_dword(base, (size * 2) as u32);
    if str_ptr != 0 {
        emu.maps.memcpy(base + 8, str_ptr, (size * 2) as usize);
    }

    log_red!(emu, "oleaut32!SysAllocStringLen ={} chars:{}", name, size);

    emu.regs_mut().rax = base + 8;
    stdcall_ret(emu, 2);
}

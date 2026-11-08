use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn SysReAllocStringLen(emu: &mut emu::Emu) {
    let bstr_ptr = arg(emu, 0) as u64;
    let new_str = arg(emu, 1) as u64;
    let new_len = arg(emu, 2) as u64;

    log_red!(
        emu,
        "oleaut32!SysReAllocStringLen ptr:0x{:x} chars:{}",
        bstr_ptr,
        new_len
    );

    let alloc_size = new_len * 2 + 2 + 8;
    let base = match emu.maps.alloc(alloc_size) {
        Some(a) => a,
        None => {
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 3);
            return;
        }
    };
    let name = format!("alloc_{:x}", base + 8);
    emu.maps
        .create_map(&name, base, alloc_size, Permission::READ_WRITE);
    emu.maps.write_dword(base, (new_len * 2) as u32);
    if new_str != 0 {
        emu.maps.memcpy(base + 8, new_str, (new_len * 2) as usize);
    }
    if bstr_ptr != 0 {
        emu.maps.write_dword(bstr_ptr, (base + 8) as u32);
    }

    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

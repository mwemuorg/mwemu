use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn SysAllocString(emu: &mut emu::Emu) {
    let str_ptr = arg(emu, 0) as u64;

    let s = if str_ptr != 0 {
        emu.maps.read_wide_string(str_ptr)
    } else {
        String::new()
    };
    let byte_len = (s.len() * 2) as u64;
    let alloc_size = byte_len + 2 + 8;

    let base = match emu.maps.alloc(alloc_size) {
        Some(a) => a,
        None => {
            log::error!("oleaut32!SysAllocString out of memory");
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
            return;
        }
    };
    let name = format!("alloc_{:x}", base + 8);
    emu.maps
        .create_map(&name, base, alloc_size, Permission::READ_WRITE);
    emu.maps.write_dword(base, byte_len as u32);
    if str_ptr != 0 {
        emu.maps.memcpy(base + 8, str_ptr, byte_len as usize);
    }

    log_red!(emu, "oleaut32!SysAllocString \'{}\'  ={}", s, name);

    emu.regs_mut().rax = base + 8;
    stdcall_ret(emu, 1);
}

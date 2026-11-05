use crate::api::windows::common::fallback;
use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::winapi32::kernel32;
//use crate::winapi::helper;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "SysAllocStringLen" => SysAllocStringLen(emu),
        "SysFreeString" => SysFreeString(emu),

        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}

fn SysAllocStringLen(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let mut size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    if size == 0xffffffff {
        size = 1024;
    }
    size += 1; // null byte
    size += 8; // metadata

    let base = match emu.maps.alloc(size + 8) {
        Some(a) => a,
        None => {
            log::error!("oleaut32!SysAllocStringLen out of memory");
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
            return;
        }
    };
    let name = format!("alloc_{:x}", base + 8);
    emu.maps
        .create_map(&name, base, size, Permission::READ_WRITE);
    emu.maps.memcpy(base + 8, str_ptr, size as usize - 1);

    log_red!(emu, "oleaut32!SysAllocStringLen  ={} {}", name, size - 8);

    for _ in 0..2 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = base + 8;
}

fn SysFreeString(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    log_red!(emu, "oleaut32!SysFreeString  0x{:x}", str_ptr);

    //emu.maps.free(&format!("alloc_{:x}", str_ptr));

    emu.stack_pop32(false);
}

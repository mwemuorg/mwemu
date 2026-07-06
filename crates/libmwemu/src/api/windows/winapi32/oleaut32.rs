use crate::emu;
use crate::maps::mem64::Permission;
use crate::serialization;
use crate::winapi::winapi32::kernel32;
//use crate::winapi::helper;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "SysAllocStringLen" => SysAllocStringLen(emu),
        "SysFreeString" => SysFreeString(emu),

        _ => {
            if !emu.cfg.skip_unimplemented {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump(
                        emu,
                        emu.cfg.dump_filename.as_ref().unwrap(),
                    );
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
            }
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
        }
    }

    String::new()
}

fn SysAllocStringLen(emu: &mut emu::Emu) {
    let str_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("oleaut32!SysAllocStringLen cannot read str_ptr") as u64;
    let mut size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("oleaut32!SysAllocStringLen cannot read size") as u64;

    if size == 0xffffffff {
        size = 1024;
    }
    size += 1; // null byte
    size += 8; // metadata

    let base = emu
        .maps
        .alloc(size + 8)
        .expect("oleaut32!SysAllocStringLen out of memory");
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
    let str_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("oleaut32!SysFreeString cannot read host_port") as u64;

    log_red!(emu, "oleaut32!SysFreeString  0x{:x}", str_ptr);

    //emu.maps.free(&format!("alloc_{:x}", str_ptr));

    emu.stack_pop32(false);
}

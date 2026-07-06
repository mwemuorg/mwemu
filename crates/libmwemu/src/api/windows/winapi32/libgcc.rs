use crate::emu;
use crate::maps::mem64::Permission;
//use crate::windows::constants::*;
//use crate::winapi::helper;
use crate::serialization;
use crate::winapi::winapi32::kernel32;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "__register_frame_info" => __register_frame_info(emu),
        "__deregister_frame_info" => __deregister_frame_info(emu),

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

fn __register_frame_info(emu: &mut emu::Emu) {
    let p1 = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("advapi32!__register_frame_info error reading param");
    let p2 = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("advapi32!__register_frame_info error reading param");

    log_red!(emu, "libgcc!__register_frame_info {:x} {:x}", p1, p2);

    let mem = match emu.maps.get_mem_by_addr_mut(0x40E198) {
        Some(m) => m,
        None => emu
            .maps
            .create_map("glob1", 0x40E198, 100, Permission::READ_WRITE_EXECUTE)
            .expect("cannot create glob1 map"),
    };

    mem.write_dword(0x40E198, 0x6e940000);

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn __deregister_frame_info(emu: &mut emu::Emu) {
    let p1 = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("advapi32!__deregister_frame_info error reading param");

    log_red!(emu, "libgcc!__deregister_frame_info {:x}", p1);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}

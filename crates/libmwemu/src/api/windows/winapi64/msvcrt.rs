use crate::emu;
use crate::maps::mem64::Permission;
use crate::serialization;
use crate::winapi::winapi64::kernel32;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);

    gateway_by_name(api, emu)
}

pub fn gateway_by_name(api: &str, emu: &mut emu::Emu) -> String {
    match api {
        "__set_app_type" => __set_app_type(emu),
        "malloc" => malloc(emu),
        "_errno" => _errno(emu),
        _ => {
            if !emu.cfg.skip_unimplemented {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump(
                        emu,
                        emu.cfg.dump_filename.as_ref().unwrap(),
                    );
                }

                unimplemented!("atemmpt to call unimplemented msvcrt API {}", api);
            }
            log::warn!(
                "calling unimplemented msvcrt API {} at 0x{:x}",
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
        }
    }

    String::new()
}

/*
void __set_app_type (
   int at
)
*/
fn __set_app_type(emu: &mut emu::Emu) {
    let app_type = emu.regs().rcx;
    log_red!(
        emu,
        "** {} msvcrt!__set_app_type  app_type: 0x{:x}",
        emu.pos,
        app_type
    );
}

fn malloc(emu: &mut emu::Emu) {
    let size = emu.regs().rcx;

    if size > 0 {
        let base = emu.maps.alloc(size).expect("msvcrt!malloc out of memory");

        emu.maps
            .create_map(
                &format!("alloc_{:x}", base),
                base,
                size,
                Permission::READ_WRITE,
            )
            .expect("msvcrt!malloc cannot create map");

        log_red!(emu, "msvcrt!malloc sz: {} addr: 0x{:x}", size, base);

        emu.regs_mut().rax = base;
    } else {
        emu.regs_mut().rax = 0x1337; // weird msvcrt has to return a random unallocated pointer, and the program has to do free() on it
    }
}

fn _errno(emu: &mut emu::Emu) {
    log_red!(emu, "msvcrt!_errno =0");
    emu.regs_mut().rax = 0;
}

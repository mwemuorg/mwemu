use crate::emu;
use crate::serialization;
use crate::winapi::winapi32::kernel32;
//use crate::winapi::helper;
//use crate::endpoint;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "DnsQuery_A" => DnsQuery_A(emu),
        "DnsQueryA" => DnsQuery_A(emu),
        "DnsQuery_W" => DnsQuery_W(emu),
        "DnsQueryW" => DnsQuery_W(emu),

        _ => {
            if emu.cfg.skip_unimplemented {
                let nparams = emu.banzai.get_params(api);
                if nparams >= 0 {
                    for _ in 0..nparams {
                        emu.stack_pop32(false);
                    }
                }
                log::warn!(
                    "banzai: skipping unimplemented API 0x{:x} {} (nparams={}) at 0x{:x}",
                    addr,
                    api,
                    nparams,
                    emu.regs().rip
                );
                emu.regs_mut().rax = 0;
                return String::new();
            }

            if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                serialization::Serialization::dump(
                    emu,
                    emu.cfg.dump_filename.as_deref().unwrap_or("mwemu_dump.bin"),
                );
            }

            log::error!("attempt to call unimplemented API 0x{:x} {}", addr, api);
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

fn DnsQuery_A(emu: &mut emu::Emu) {
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let wtype = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let opt = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let extra = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let out_results = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let out_reserved = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);

    let name = emu.maps.read_string(name_ptr);

    log_red!(emu, "dnsapi!DnsQuery_A '{}'", name);

    emu.regs_mut().rax = 1;
}

fn DnsQuery_W(emu: &mut emu::Emu) {
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let wtype = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let opt = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let extra = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let out_results = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let out_reserved = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);

    let name = emu.maps.read_wide_string(name_ptr);

    log_red!(emu, "dnsapi!DnsQuery_W '{}'", name);

    emu.regs_mut().rax = 1;
}

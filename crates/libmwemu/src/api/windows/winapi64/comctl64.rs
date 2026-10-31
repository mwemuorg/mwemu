use crate::emu;
use crate::serialization;
use crate::winapi::winapi64::kernel32;
//use crate::winapi::helper;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    if emu.cfg.skip_unimplemented {
        log::warn!(
            "banzai: skipping unimplemented API 0x{:x} {} at 0x{:x}",
            addr,
            api,
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
    api.to_ascii_lowercase()
}

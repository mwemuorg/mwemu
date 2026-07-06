use crate::emu;
use crate::serialization;
use crate::winapi::winapi32::kernel32;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    {
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
        api.to_ascii_lowercase()
    }
}

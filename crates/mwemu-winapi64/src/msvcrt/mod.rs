use crate::emu;
use crate::maps::buffers::BufferKind;
use crate::serialization;
use crate::winapi::common::buffers;
use crate::winapi::common::crt;
use crate::winapi::winapi64::kernel32;

mod runtime;
pub use runtime::*;
mod strings;
pub use strings::*;
mod io;
pub use io::*;
mod memory;
pub use memory::*;
mod math;
pub use math::*;

fn allocate_memory(emu: &mut emu::Emu, size: u64) -> Option<u64> {
    buffers::allocate(emu, size, 16, 0, BufferKind::Crt)
}

fn release(emu: &mut emu::Emu, addr: u64) {
    buffers::free(emu, addr, BufferKind::Crt);
}

// --- 64-bit CRT gateway ---

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    gateway_by_name(api, emu)
}

pub fn gateway_by_name(api: &str, emu: &mut emu::Emu) -> String {
    if crt::dispatch(api, emu) {
        return String::new();
    }
    if runtime::dispatch(api, emu)
        || strings::dispatch(api, emu)
        || io::dispatch(api, emu)
        || memory::dispatch(api, emu)
        || math::dispatch(api, emu)
    {
        return String::new();
    }

    let rip = emu.regs().rip;
    if emu.cfg.skip_unimplemented {
        let nparams = emu.banzai.get_params(api);
        log::warn!(
            "banzai: skipping unimplemented API {} (nparams={}) at 0x{:x}",
            api,
            nparams,
            rip
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

    log::error!("attempt to call unimplemented API {}", api);
    log::warn!("calling unimplemented API {} at 0x{:x}", api, rip);
    api.to_ascii_lowercase()
}

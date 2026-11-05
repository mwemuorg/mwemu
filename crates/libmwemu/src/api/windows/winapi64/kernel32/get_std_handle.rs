use crate::emu;
use crate::winapi::helper;

pub fn GetStdHandle(emu: &mut emu::Emu) {
    let nstd = emu.regs().rcx as u32;

    let handle = match nstd {
        0xFFFFFFF6 => {
            // STD_INPUT_HANDLE (-10)
            let uri = "stdio://stdin";
            helper::handler_find_by_uri(uri).unwrap_or_else(|| helper::handler_create(uri))
        }
        0xFFFFFFF5 => {
            // STD_OUTPUT_HANDLE (-11)
            let uri = "stdio://stdout";
            helper::handler_find_by_uri(uri).unwrap_or_else(|| helper::handler_create(uri))
        }
        0xFFFFFFF4 => {
            // STD_ERROR_HANDLE (-12)
            let uri = "stdio://stderr";
            helper::handler_find_by_uri(uri).unwrap_or_else(|| helper::handler_create(uri))
        }
        _ => {
            emu.set_last_error(6); // ERROR_INVALID_HANDLE
            0xFFFF_FFFF_FFFF_FFFF // INVALID_HANDLE_VALUE
        }
    };

    log_red!(
        emu,
        "kernel32!GetStdHandle nstd:0x{:x} => 0x{:x}",
        nstd,
        handle
    );

    emu.regs_mut().rax = handle;
}

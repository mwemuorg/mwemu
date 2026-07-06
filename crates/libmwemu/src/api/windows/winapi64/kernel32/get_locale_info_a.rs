use crate::winapi::winapi64::kernel32::{LAST_ERROR, clear_last_error};
use crate::{emu, windows::constants};

pub fn GetLocaleInfoA(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx as usize;
    let lctype = emu.regs().rdx as usize;
    let lp_lc_data = emu.regs().r8 as usize;
    let cch_data = emu.regs().r9 as usize;

    let result = ".";
    let required_size = result.len() + 1; // Include null terminator

    // If cchData is 0, return required buffer size
    if cch_data == 0 {
        emu.regs_mut().rax = required_size as u64;
        clear_last_error(emu);
        return;
    }

    // Check if buffer is too small
    if cch_data < required_size {
        log::warn!(
            "{} buffer too small for result cch_data: {} required_size: {}",
            emu.pos,
            cch_data,
            required_size
        );
        let mut err = LAST_ERROR.lock().unwrap();
        *err = constants::ERROR_INSUFFICIENT_BUFFER;
        emu.regs_mut().rax = 0;
        return;
    }

    // Write result directly to provided buffer
    emu.maps.write_string(lp_lc_data as u64, result);
    emu.regs_mut().rax = result.len() as u64; // Return length without null terminator
    clear_last_error(emu);
}

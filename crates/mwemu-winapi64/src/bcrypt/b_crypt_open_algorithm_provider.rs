use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::errors::*;

pub fn BCryptOpenAlgorithmProvider(emu: &mut emu::Emu) {
    let ph_algorithm = emu.regs().rcx;
    let psz_alg_id = emu.regs().rdx;
    let flags = emu.regs().r9;

    let algo_name = emu.maps.read_wide_string(psz_alg_id);

    log_red!(
        emu,
        "bcrypt!BCryptOpenAlgorithmProvider algo: {} flags: 0x{:x}",
        algo_name,
        flags
    );

    let handle = helper::handler_create(&format!("bcrypt://{}", algo_name));
    if ph_algorithm != 0 {
        emu.maps.write_qword(ph_algorithm, handle);
    }
    emu.regs_mut().rax = STATUS_SUCCESS;
}

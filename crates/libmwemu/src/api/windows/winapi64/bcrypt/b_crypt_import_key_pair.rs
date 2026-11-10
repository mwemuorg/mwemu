use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::errors::*;

pub fn BCryptImportKeyPair(emu: &mut emu::Emu) {
    let h_algorithm = emu.regs().rcx;
    let _h_import_key = emu.regs().rdx;
    let _psz_blob_type = emu.regs().r8;
    let ph_key = emu.regs().r9;

    let algo_uri = helper::handler_get_uri(h_algorithm);
    let algo_name = algo_uri.strip_prefix("bcrypt://").unwrap_or(&algo_uri);
    let hndl = helper::handler_create(&format!("bcryptkey://{}", algo_name));

    if ph_key != 0 {
        emu.maps.write_qword(ph_key, hndl);
    }

    log_red!(
        emu,
        "bcrypt!BCryptImportKeyPair algo={} =0x{:x}",
        algo_name,
        hndl
    );

    emu.regs_mut().rax = STATUS_SUCCESS;
}

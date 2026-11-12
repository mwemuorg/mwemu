use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub(super) fn BCryptImportKeyPair(emu: &mut emu::Emu) {
    let h_algorithm = arg(emu, 0) as u64;
    let _h_import_key = arg(emu, 1) as u64;
    let _psz_blob_type = arg(emu, 2) as u64;
    let ph_key = arg(emu, 3) as u64;
    let _pb_input = arg(emu, 4) as u64;
    let _cb_input = arg(emu, 5);

    let algo_uri = helper::handler_get_uri(h_algorithm);
    let algo_name = algo_uri.strip_prefix("bcrypt://").unwrap_or(&algo_uri);
    let hndl = helper::handler_create(&format!("bcryptkey://{}", algo_name));

    if ph_key != 0 {
        emu.maps.write_dword(ph_key, hndl as u32);
    }

    log_red!(
        emu,
        "bcrypt!BCryptImportKeyPair algo={} =0x{:x}",
        algo_name,
        hndl
    );

    emu.regs_mut().rax = STATUS_SUCCESS;
    stdcall_ret(emu, 7);
}

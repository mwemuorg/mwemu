use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub(super) fn BCryptExportKey(emu: &mut emu::Emu) {
    let _h_key = arg(emu, 0) as u64;
    let _h_export_key = arg(emu, 1) as u64;
    let psz_blob_type = arg(emu, 2) as u64;
    let pb_output = arg(emu, 3) as u64;
    let cb_output = arg(emu, 4) as usize;
    let pcb_result = arg(emu, 5) as u64;

    let blob_type = emu.maps.read_wide_string(psz_blob_type);
    log_red!(emu, "bcrypt!BCryptExportKey type={}", blob_type);

    if pb_output != 0 && cb_output > 0 {
        emu.maps.memset(pb_output, 0, cb_output.min(1024));
    }
    if pcb_result != 0 {
        emu.maps.write_dword(pcb_result, 0);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
    stdcall_ret(emu, 7);
}

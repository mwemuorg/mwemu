use crate::emu;
use crate::windows::constants;

pub fn BCryptExportKey(emu: &mut emu::Emu) {
    let _h_key = emu.regs().rcx;
    let _h_export_key = emu.regs().rdx;
    let psz_blob_type = emu.regs().r8;
    let pb_output = emu.regs().r9;
    let cb_output = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0) as usize;
    let pcb_result = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let blob_type = emu.maps.read_wide_string(psz_blob_type);
    log_red!(emu, "bcrypt!BCryptExportKey type={}", blob_type);

    if pb_output != 0 && cb_output > 0 {
        emu.maps.memset(pb_output, 0, cb_output.min(1024));
    }
    if pcb_result != 0 {
        emu.maps.write_dword(pcb_result, 0);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

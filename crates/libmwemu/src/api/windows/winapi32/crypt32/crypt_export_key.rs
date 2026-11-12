use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptExportKey(emu: &mut emu::Emu) {
    let h_key = WinApiAbi::arg(emu, 0);
    let h_exp_key = WinApiAbi::arg(emu, 1);
    let dw_blob_type = WinApiAbi::arg(emu, 2) as u32;
    let dw_flags = WinApiAbi::arg(emu, 3) as u32;
    let pb_data = WinApiAbi::arg(emu, 4);
    let pdw_data_len = WinApiAbi::arg(emu, 5);
    log_red!(
        emu,
        "crypt32!CryptExportKey key:0x{:x} exp:0x{:x} type:{} flags:0x{:x}",
        h_key,
        h_exp_key,
        dw_blob_type,
        dw_flags
    );
    let val_ptr = WinApiAbi::arg(emu, 1);
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
}

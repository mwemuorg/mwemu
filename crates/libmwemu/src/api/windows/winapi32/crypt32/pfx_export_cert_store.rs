use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn PFXExportCertStore(emu: &mut emu::Emu) {
    let h_store = WinApiAbi::arg(emu, 0);
    let p_pfx = WinApiAbi::arg(emu, 1);
    let sz_password = WinApiAbi::arg(emu, 2);
    let dw_flags = WinApiAbi::arg(emu, 3);
    log_red!(emu, "crypt32!PFXExportCertStore 0x{:x}", h_store);
    let val_ptr = WinApiAbi::arg(emu, 1);
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

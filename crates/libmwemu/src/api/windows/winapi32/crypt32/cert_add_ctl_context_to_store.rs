use crate::emu;
use crate::winapi::helper;

pub fn CertAddCTLContextToStore(emu: &mut emu::Emu) {
    let h_cert_store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p_ctl_ctx = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_add_disposition = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let pp_store_ctx = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CertAddCTLContextToStore 0x{:x}", h_cert_store);
    let handle = helper::handler_create("CertAddCTLContextToStore");
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, handle as u32);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

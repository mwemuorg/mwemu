use crate::emu;

pub fn CoSetProxyBlanket(emu: &mut emu::Emu) {
    let proxy = emu.regs().rcx;
    let authn_svc = emu.regs().rdx;
    let authz_svc = emu.regs().r8;
    let server_princ = emu.regs().r9;
    log_red!(
        emu,
        "ole32!CoSetProxyBlanket proxy:0x{:x} authn:{} authz:{}",
        proxy,
        authn_svc,
        authz_svc
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..8 {}
}

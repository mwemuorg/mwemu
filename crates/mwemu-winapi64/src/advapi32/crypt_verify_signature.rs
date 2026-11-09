use crate::emu;

pub fn CryptVerifySignatureA(emu: &mut emu::Emu) {
    let hhash = emu.regs().rcx;
    let sig_ptr = emu.regs().rdx;
    let sig_len = emu.regs().r8;
    let hpub_key = emu.regs().r9;
    let desc_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _flags = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let desc = if desc_ptr != 0 {
        emu.maps.read_string(desc_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "advapi32!CryptVerifySignatureA hash:0x{:x} sig:0x{:x} len:{} key:0x{:x} desc:'{}'",
        hhash,
        sig_ptr,
        sig_len,
        hpub_key,
        desc
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn CryptVerifySignatureW(emu: &mut emu::Emu) {
    let hhash = emu.regs().rcx;
    let sig_ptr = emu.regs().rdx;
    let sig_len = emu.regs().r8;
    let hpub_key = emu.regs().r9;
    let desc_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _flags = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let desc = if desc_ptr != 0 {
        emu.maps.read_wide_string(desc_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "advapi32!CryptVerifySignatureW hash:0x{:x} sig:0x{:x} len:{} key:0x{:x} desc:'{}'",
        hhash,
        sig_ptr,
        sig_len,
        hpub_key,
        desc
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

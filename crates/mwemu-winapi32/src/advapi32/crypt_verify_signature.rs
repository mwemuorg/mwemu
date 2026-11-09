use crate::emu;

pub fn CryptVerifySignatureA(emu: &mut emu::Emu) {
    let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let sig_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let sig_len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let hpub_key = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let desc_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
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
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn CryptVerifySignatureW(emu: &mut emu::Emu) {
    let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let sig_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let sig_len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let hpub_key = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let desc_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
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
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

use crate::emu;
use crate::winapi::common::strings;

pub fn CredUnmarshalCredentialA(emu: &mut emu::Emu) {
    cred_unmarshal_credential(emu, "CredUnmarshalCredentialA", false);
}

fn cred_unmarshal_credential(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "advapi32!{api} '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn CredUnmarshalCredentialW(emu: &mut emu::Emu) {
    cred_unmarshal_credential(emu, "CredUnmarshalCredentialW", true);
}

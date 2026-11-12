use crate::emu;
use crate::winapi::common::strings;

pub fn GetExplicitEntriesFromAclA(emu: &mut emu::Emu) {
    get_explicit_entries_from_acl(emu, "GetExplicitEntriesFromAclA", false);
}

fn get_explicit_entries_from_acl(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "advapi32!{api} '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GetExplicitEntriesFromAclW(emu: &mut emu::Emu) {
    get_explicit_entries_from_acl(emu, "GetExplicitEntriesFromAclW", true);
}

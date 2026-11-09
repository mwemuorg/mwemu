use crate::emu;

pub(super) fn CharLowerW(emu: &mut emu::Emu) {
    let ptr_str = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    let s = emu.maps.read_wide_string(ptr_str);

    log_red!(emu, "user32!CharLowerW(`{}`)", s);

    emu.maps.write_wide_string(ptr_str, &s.to_lowercase());

    emu.stack_pop32(false);
    emu.regs_mut().rax = ptr_str;
}

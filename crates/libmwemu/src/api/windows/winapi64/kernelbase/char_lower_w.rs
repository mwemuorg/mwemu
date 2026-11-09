use crate::emu;

pub fn CharLowerW(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    if str_ptr > 0xffff {
        let s = emu.maps.read_wide_string(str_ptr);
        let lower = s.to_lowercase();
        emu.maps.write_wide_string(str_ptr, &lower);
        log_red!(emu, "kernelbase!CharLowerW '{}' => '{}'", s, lower);
        emu.regs_mut().rax = str_ptr;
    } else {
        let ch = (str_ptr as u16) as u32;
        let lowered = char::from_u32(ch)
            .map(|c| c.to_lowercase().next().unwrap_or(c) as u64)
            .unwrap_or(str_ptr);
        log_red!(emu, "kernelbase!CharLowerW char:0x{:x}", str_ptr);
        emu.regs_mut().rax = lowered;
    }
}

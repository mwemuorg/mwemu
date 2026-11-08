use crate::emu;

pub fn CharUpperW(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    if str_ptr > 0xffff {
        let s = emu.maps.read_wide_string(str_ptr);
        let upper = s.to_uppercase();
        emu.maps.write_wide_string(str_ptr, &upper);
        log_red!(emu, "kernelbase!CharUpperW '{}' => '{}'", s, upper);
        emu.regs_mut().rax = str_ptr;
    } else {
        let ch = (str_ptr as u16) as u32;
        let uppered = char::from_u32(ch)
            .map(|c| c.to_uppercase().next().unwrap_or(c) as u64)
            .unwrap_or(str_ptr);
        log_red!(emu, "kernelbase!CharUpperW char:0x{:x}", str_ptr);
        emu.regs_mut().rax = uppered;
    }
}

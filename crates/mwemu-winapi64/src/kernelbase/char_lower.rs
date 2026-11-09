use crate::emu;

pub fn CharLowerA(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    if str_ptr > 0xffff {
        let s = emu.maps.read_string(str_ptr);
        let lower = s.to_lowercase();
        emu.maps.write_string(str_ptr, &lower);
        log_red!(emu, "kernelbase!CharLowerA '{}' => '{}'", s, lower);
        emu.regs_mut().rax = str_ptr;
    } else {
        let ch = str_ptr as u8;
        let lowered = (ch as char).to_lowercase().next().unwrap_or(ch as char) as u64;
        log_red!(emu, "kernelbase!CharLowerA char:0x{:x}", str_ptr);
        emu.regs_mut().rax = lowered;
    }
}

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

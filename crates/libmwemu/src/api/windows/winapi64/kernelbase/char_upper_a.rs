use crate::emu;

pub fn CharUpperA(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    if str_ptr > 0xffff {
        let s = emu.maps.read_string(str_ptr);
        let upper = s.to_uppercase();
        emu.maps.write_string(str_ptr, &upper);
        log_red!(emu, "kernelbase!CharUpperA '{}' => '{}'", s, upper);
        emu.regs_mut().rax = str_ptr;
    } else {
        let ch = str_ptr as u8;
        let uppered = (ch as char).to_uppercase().next().unwrap_or(ch as char) as u64;
        log_red!(emu, "kernelbase!CharUpperA char:0x{:x}", str_ptr);
        emu.regs_mut().rax = uppered;
    }
}

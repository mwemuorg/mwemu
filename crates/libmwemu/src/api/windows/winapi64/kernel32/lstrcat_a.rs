use crate::emu;

pub fn lstrcatA(emu: &mut emu::Emu) {
    let str1_ptr = emu.regs().rcx;
    let str2_ptr = emu.regs().rdx;

    let mut str1 = emu.maps.read_string(str1_ptr);
    let str2 = emu.maps.read_string(str2_ptr);

    log_red!(emu, "kernel32!lstrcatA '{}'+'{}'", str1, str2);

    str1.push_str(&str2);
    emu.maps.write_string(str1_ptr, &str1);

    emu.regs_mut().rax = 1;
}

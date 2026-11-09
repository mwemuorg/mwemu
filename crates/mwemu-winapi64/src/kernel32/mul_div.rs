use crate::emu;

pub fn MulDiv(emu: &mut emu::Emu) {
    let number = emu.regs().rcx as i32;
    let numerator = emu.regs().rdx as i32;
    let denominator = emu.regs().r8 as i32;

    log_red!(
        emu,
        "kernel32!MulDiv number:{} numerator:{} denominator:{}",
        number,
        numerator,
        denominator
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    if denominator == 0 {
        emu.set_last_error(0);
        emu.regs_mut().rax = -1i64 as u64;
        return;
    }

    let result = (number as i64 * numerator as i64) / denominator as i64;
    if result > i32::MAX as i64 || result < i32::MIN as i64 {
        emu.regs_mut().rax = -1i64 as u64;
    } else {
        emu.regs_mut().rax = result as i32 as u64;
    }
}

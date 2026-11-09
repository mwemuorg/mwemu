use crate::emu;

pub fn ULongMult(emu: &mut emu::Emu) {
    let a = emu.regs().rcx as u32;
    let b = emu.regs().rdx as u32;
    let p_result = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!ULongMult a:{} b:{} pResult:0x{:x}",
        a,
        b,
        p_result
    );

    match a.checked_mul(b) {
        Some(product) => {
            if p_result != 0 {
                emu.maps.write_dword(p_result, product as u32);
            }
            emu.regs_mut().rax = 0;
        }
        None => {
            emu.regs_mut().rax = 0x80070216;
        }
    }
}

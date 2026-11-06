use crate::emu;

pub fn ULongAdd(emu: &mut emu::Emu) {
    let a = emu.regs().rcx as u32;
    let b = emu.regs().rdx as u32;
    let p_result = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!ULongAdd a:{} b:{} pResult:0x{:x}",
        a,
        b,
        p_result
    );

    match a.checked_add(b) {
        Some(sum) => {
    if p_result != 0 {
        emu.maps.write_dword(p_result, sum as u64);
    }
    emu.regs_mut().rax = 0;
        }
        None => {
    emu.regs_mut().rax = 0x80070216;
        }
    }
}

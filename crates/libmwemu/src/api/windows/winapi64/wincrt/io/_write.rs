use crate::emu;

pub fn _write(emu: &mut emu::Emu) {
    let fd = emu.regs().rcx as i32;
    let buf = emu.regs().rdx;
    let count = emu.regs().r8 as u32;
    let data = emu.maps.read_string(buf);
    log_red!(
        emu,
        "wincrt!_write fd:{} count:{} data:'{}'",
        fd,
        count,
        data
    );
    emu.regs_mut().rax = count as u64;
}

use crate::emu;

pub fn CopyLZFile(emu: &mut emu::Emu) {
    let hf_source = emu.regs().rcx;
    let hf_dest = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!CopyLZFile src:0x{:x} dest:0x{:x}",
        hf_source,
        hf_dest
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = (-1i64) as u64;
}

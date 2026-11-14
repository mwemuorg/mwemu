use crate::emu;

pub fn __FC_Query_Config(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    log_red!(
        emu,
        "kernelbase!__FC_Query_Config {}:0x{:x} {}:0x{:x}",
        "param1",
        param1,
        "param2",
        param2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // success
}

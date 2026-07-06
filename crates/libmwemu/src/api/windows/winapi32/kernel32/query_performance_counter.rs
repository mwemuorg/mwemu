use crate::emu;

pub fn QueryPerformanceCounter(emu: &mut emu::Emu) {
    let counter_ptr =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("kernel32!QueryPerformanceCounter cannot read counter_ptr") as u64;

    emu.maps.write_dword(counter_ptr, 0x1);

    log_red!(emu, "kernel32!QueryPerformanceCounter");

    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}

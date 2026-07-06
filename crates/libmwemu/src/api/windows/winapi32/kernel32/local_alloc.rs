use crate::emu;
use crate::maps::mem64::Permission;

pub fn LocalAlloc(emu: &mut emu::Emu) {
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!LocalAlloc cannot read flags");
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!LocalAlloc cannot read size") as u64;

    emu.regs_mut().rax = emu.maps.alloc(size).unwrap_or_default();

    emu.maps
        .create_map(
            format!("alloc_{:x}", emu.regs().get_eax() as u32).as_str(),
            emu.regs().get_eax(),
            size,
            Permission::READ_WRITE,
        )
        .expect("kernel32!LocalAlloc out of memory");

    log_red!(
        emu,
        "kernel32!LocalAlloc flags: 0x{:x} size: {} =0x{:x}",
        flags,
        size,
        emu.regs().get_eax() as u32
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);
}

use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::helper::WinApiAbi;

pub(super) fn __register_frame_info(emu: &mut emu::Emu) {
    let p1 = WinApiAbi::arg(emu, 0) as u32;
    let p2 = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "libgcc!__register_frame_info {:x} {:x}", p1, p2);

    let mem = match emu.maps.get_mem_by_addr_mut(0x40E198) {
        Some(m) => m,
        None => match emu
            .maps
            .create_map("glob1", 0x40E198, 100, Permission::READ_WRITE_EXECUTE)
        {
            Ok(m) => m,
            Err(e) => {
                log::error!(
                    "libgcc!__register_frame_info: cannot create glob1 map: {}",
                    e
                );
                for _ in 0..2 {
                    emu.stack_pop32(false);
                }
                return;
            }
        },
    };

    mem.write_dword(0x40E198, 0x6e940000);

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

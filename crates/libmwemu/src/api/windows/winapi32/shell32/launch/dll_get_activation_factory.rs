use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn DllGetActivationFactory(emu: &mut emu::Emu) {
    let class_id_ptr = WinApiAbi::arg(emu, 0);
    let iid_ptr = WinApiAbi::arg(emu, 1);
    let factory_ptr = WinApiAbi::arg(emu, 2);
    log_red!(emu, "shell32!DllGetActivationFactory");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x80004001u64;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}

use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn TlsGetValue(emu: &mut emu::Emu) {
    let idx = WinApiAbi::arg(emu, 0) as u32;

    emu.stack_pop32(false);

    if idx as usize >= emu.tls32().len() {
        emu.regs_mut().set_eax(0);
    } else {
        let tls_entry = emu.tls32()[idx as usize] as u64;
        emu.regs_mut().set_eax(tls_entry);
    }

    log_red!(
        emu,
        "** {} kernel32!TlsGetValue idx: {} =0x{:x}",
        emu.pos,
        idx,
        emu.regs().get_eax() as u32
    );
}

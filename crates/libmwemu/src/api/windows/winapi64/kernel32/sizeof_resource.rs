use crate::emu;
use crate::winapi::helper;

pub fn SizeofResource(emu: &mut emu::Emu) {
    let hModule = emu.regs().rcx;
    let hResInfo = emu.regs().rdx;

    if helper::handler_exist(hResInfo) {
        let uri = helper::handler_get_uri(hResInfo);
        let size = match uri.split("_").last().and_then(|s| s.parse::<usize>().ok()) {
            Some(v) => v,
            None => {
                log::warn!("SizeofResource: bad handler uri `{}`", uri);
                emu.regs_mut().rax = 0;
                return;
            }
        };
        log::trace!(
            "** {} kernel32!SizeofResource {:x} {:x} size: {}",
            emu.pos,
            hModule,
            hResInfo,
            size
        );
        emu.regs_mut().rax = size as u64;
        return;
    }

    log_red!(
        emu,
        "** {} kernel32!SizeofResource {:x} {:x} not found",
        emu.pos,
        hModule,
        hResInfo
    );

    emu.regs_mut().rax = 0;
}

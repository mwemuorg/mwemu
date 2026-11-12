use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;

pub fn FindResourceA(emu: &mut emu::Emu) {
    find_resource(emu, "FindResourceA", false);
}

fn find_resource(emu: &mut emu::Emu, api: &str, wide: bool) {
    let handle = arg(emu, 0) as usize;
    let lpName = arg(emu, 1) as usize;
    let lpType = arg(emu, 2) as usize;

    let (pe, raw) = match (emu.pe32.as_ref(), emu.pe32_raw.as_deref()) {
        (Some(pe), Some(raw)) => (pe, raw),
        _ => {
            log::warn!("kernel32!{api}: PE32 not loaded");
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
            return;
        }
    };

    let x: Option<(u64, usize)> = if lpName > 0xff && lpType > 0xff {
        let name = strings::read_text(emu, lpName as u64, wide);
        let ntype = strings::read_text(emu, lpType as u64, wide);
        log_red!(
            emu,
            "** {} kernel32!{api} {:x} `{}` `{}`",
            emu.pos,
            handle,
            name,
            ntype
        );
        pe.get_resource(raw, None, None, Some(name.as_str()), Some(ntype.as_str()))
    } else if lpName > 0xff && lpType <= 0xff {
        let name = strings::read_text(emu, lpName as u64, wide);
        log_red!(
            emu,
            "** {} kernel32!{api} {:x} `{}` {}",
            emu.pos,
            handle,
            name,
            lpType
        );
        pe.get_resource(raw, Some(lpType as u32), None, Some(name.as_str()), None)
    } else if lpName <= 0xff && lpType > 0xff {
        let ntype = strings::read_text(emu, lpType as u64, wide);
        log_red!(
            emu,
            "** {} kernel32!{api} {:x} `{}` {}",
            emu.pos,
            handle,
            lpName,
            ntype
        );
        pe.get_resource(raw, None, Some(lpName as u32), None, Some(ntype.as_str()))
    } else {
        log_red!(
            emu,
            "** {} kernel32!{api} {:x} `{}` {}",
            emu.pos,
            handle,
            lpName,
            lpType
        );
        pe.get_resource(raw, Some(lpType as u32), Some(lpName as u32), None, None)
    };

    for _ in 0..3 {
        emu.stack_pop32(false);
    }

    let Some((addr, size)) = x else {
        log::trace!("{} resource not found!", emu.pos);
        emu.regs_mut().rax = 0;
        return;
    };

    log::trace!("resource addr: 0x{:x} sz: {}", addr, size);
    let hndl = helper::handler_create(&format!("rsrc://{:x}_{}", addr, size));

    emu.regs_mut().rax = hndl;
}

pub fn FindResourceW(emu: &mut emu::Emu) {
    find_resource(emu, "FindResourceW", true);
}

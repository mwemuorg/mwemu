use crate::emu;
use crate::winapi::helper;

pub fn FindResourceA(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx as usize;
    let lpName = emu.regs().rdx as usize;
    let lpType = emu.regs().r8 as usize;

    let x: Option<(u64, usize)>;

    if lpName > 0xff && lpType > 0xff {
        let name = emu.maps.read_string(lpName as u64);
        let ntype = emu.maps.read_string(lpType as u64);
        log_red!(
            emu,
            "** {} kernel32!FindResourceA {:x} `{}` `{}`",
            emu.pos,
            handle,
            name,
            ntype
        );

        x = emu
            .pe32
            .as_ref()
            .unwrap()
            .get_resource(emu.pe32_raw.as_deref().unwrap(), None, None, Some(name.as_str()), Some(ntype.as_str()));
    } else if lpName > 0xff && lpType <= 0xff {
        let name = emu.maps.read_string(lpName as u64);
        log_red!(
            emu,
            "** {} kernel32!FindResourceA {:x} `{}` {}",
            emu.pos,
            handle,
            name,
            lpType
        );

        x = emu
            .pe32
            .as_ref()
            .unwrap()
            .get_resource(emu.pe32_raw.as_deref().unwrap(), Some(lpType as u32), None, Some(name.as_str()), None);
    } else if lpName <= 0xff && lpType > 0xff {
        let ntype = emu.maps.read_string(lpType as u64);
        log_red!(
            emu,
            "** {} kernel32!FindResourceA {:x} `{}` {}",
            emu.pos,
            handle,
            lpName,
            ntype
        );

        x = emu
            .pe32
            .as_ref()
            .unwrap()
            .get_resource(emu.pe32_raw.as_deref().unwrap(), None, Some(lpName as u32), None, Some(ntype.as_str()));
    } else if lpName <= 0xff && lpType <= 0xff {
        log_red!(
            emu,
            "** {} kernel32!FindResourceA {:x} `{}` {}",
            emu.pos,
            handle,
            lpName,
            lpType
        );

        x = emu.pe32.as_ref().unwrap().get_resource(emu.pe32_raw.as_deref().unwrap(), 
            Some(lpType as u32),
            Some(lpName as u32),
            None,
            None,
        );
    } else {
        unreachable!();
    }

    for _ in 0..3 {
        emu.stack_pop32(false);
    }

    if x.is_none() {
        log::trace!("{} resource not found!", emu.pos);
        emu.regs_mut().rax = 0;
        return;
    }

    let (addr, size) = x.unwrap();

    log::trace!("resource addr: 0x{:x} sz: {}", addr, size);
    let hndl = helper::handler_create(&format!("rsrc://{:x}_{}", addr, size));

    emu.regs_mut().rax = hndl;
}

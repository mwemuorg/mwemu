use crate::emu;
use crate::maps::mem64::Permission;

pub fn SafeArrayCreateEx(emu: &mut emu::Emu) {
    let vt = emu.regs().rcx as u16;
    let dims = emu.regs().rdx as u32;
    let rgsabound = emu.regs().r8;
    let pvextra = emu.regs().r9;
    let elem_size: u64 = match vt {
        8 => 8,   // VT_BSTR
        12 => 16, // VT_VARIANT
        _ => 4,
    };
    let total_elements = if rgsabound != 0 && dims > 0 {
        emu.maps.read_dword(rgsabound).unwrap_or(1) as u64
    } else {
        1
    };
    let data_size = total_elements * elem_size;
    let alloc_size = 32 + data_size;
    let sa = match emu.maps.alloc(alloc_size) {
        Some(a) => a,
        None => {
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            return;
        }
    };
    emu.maps.create_map(
        &format!("safearray_{:x}", sa),
        sa,
        alloc_size,
        Permission::READ_WRITE,
    );
    emu.maps.write_word(sa, dims as u16);
    emu.maps.write_word(sa + 2, 0);
    emu.maps.write_dword(sa + 4, elem_size as u32);
    emu.maps.write_dword(sa + 8, 0);
    emu.maps.write_qword(sa + 16, sa + 32);
    if rgsabound != 0 && dims > 0 {
        let count = emu.maps.read_dword(rgsabound).unwrap_or(0);
        let lbound = emu.maps.read_dword(rgsabound + 4).unwrap_or(0);
        emu.maps.write_dword(sa + 24, count);
        emu.maps.write_dword(sa + 28, lbound);
    }
    log_red!(
        emu,
        "oleaut32!SafeArrayCreateEx vt:{} dims:{} =0x{:x}",
        vt,
        dims,
        sa
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = sa;
}

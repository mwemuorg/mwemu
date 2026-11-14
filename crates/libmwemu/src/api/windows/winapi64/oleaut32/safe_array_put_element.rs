use crate::emu;

pub fn SafeArrayPutElement(emu: &mut emu::Emu) {
    let sa_ptr = emu.regs().rcx;
    let rgindices = emu.regs().rdx;
    let pv = emu.regs().r8;
    if sa_ptr != 0 && rgindices != 0 && pv != 0 {
        let elem_size = emu.maps.read_dword(sa_ptr + 4).unwrap_or(4) as u64;
        let pv_data = emu.maps.read_qword(sa_ptr + 16).unwrap_or(0);
        let lbound = emu.maps.read_dword(sa_ptr + 28).unwrap_or(0);
        let index = emu.maps.read_dword(rgindices).unwrap_or(0);
        let offset = (index.wrapping_sub(lbound)) as u64 * elem_size;
        if pv_data != 0 && elem_size <= 16 {
            let data = emu.maps.read_bytes(pv, elem_size as usize).to_vec();
            emu.maps.write_bytes(pv_data + offset, &data);
        }
    }
    log_red!(
        emu,
        "oleaut32!SafeArrayPutElement sa:0x{:x} idx_ptr:0x{:x} pv:0x{:x}",
        sa_ptr,
        rgindices,
        pv
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

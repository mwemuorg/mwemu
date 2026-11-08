use crate::emu;

pub fn GetDiskFreeSpaceW(emu: &mut emu::Emu) {
    let root_ptr = emu.regs().rcx;
    let spc_ptr = emu.regs().rdx;
    let bps_ptr = emu.regs().r8;
    let free_clusters_ptr = emu.regs().r9;
    let total_clusters_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let root = if root_ptr != 0 {
        emu.maps.read_wide_string(root_ptr)
    } else {
        String::from("C:\\")
    };
    if spc_ptr != 0 {
        emu.maps.write_dword(spc_ptr, 8);
    }
    if bps_ptr != 0 {
        emu.maps.write_dword(bps_ptr, 512);
    }
    if free_clusters_ptr != 0 {
        emu.maps.write_dword(free_clusters_ptr, 1000000);
    }
    if total_clusters_ptr != 0 {
        emu.maps.write_dword(total_clusters_ptr, 2000000);
    }
    log_red!(emu, "kernelbase!GetDiskFreeSpaceW '{}'", root);
    emu.regs_mut().rax = 1;
}

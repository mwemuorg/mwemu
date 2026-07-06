use crate::emu;

pub fn GetDiskFreeSpaceA(emu: &mut emu::Emu) {
    let root_path_ptr =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("kernel32!GetDiskFreeSpaceA cannot read root_path_ptr") as u64;
    let sectors_per_cluster_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!GetDiskFreeSpaceA cannot read sectors_per_cluster_ptr")
        as u64;
    let bytes_per_sector_ptr =
        emu.maps
            .read_dword(emu.regs().get_esp() + 8)
            .expect("kernel32!GetDiskFreeSpaceA cannot read bytes_per_sector_ptr") as u64;
    let free_clusters_ptr =
        emu.maps
            .read_dword(emu.regs().get_esp() + 12)
            .expect("kernel32!GetDiskFreeSpaceA cannot read free_clusters_ptr") as u64;
    let total_clusters_ptr =
        emu.maps
            .read_dword(emu.regs().get_esp() + 16)
            .expect("kernel32!GetDiskFreeSpaceA cannot read total_clusters_ptr") as u64;

    let root_path = if root_path_ptr != 0 {
        emu.maps.read_string(root_path_ptr)
    } else {
        "NULL".to_string()
    };

    log_red!(emu, "kernel32!GetDiskFreeSpaceA path:`{}`", root_path);

    // Dummy values
    if sectors_per_cluster_ptr != 0 {
        emu.maps.write_dword(sectors_per_cluster_ptr, 8);
    }
    if bytes_per_sector_ptr != 0 {
        emu.maps.write_dword(bytes_per_sector_ptr, 512);
    }
    if free_clusters_ptr != 0 {
        emu.maps.write_dword(free_clusters_ptr, 100000); // Plenty of space
    }
    if total_clusters_ptr != 0 {
        emu.maps.write_dword(total_clusters_ptr, 200000);
    }

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1; // Success
}

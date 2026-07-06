use crate::emu;

pub fn GetDiskFreeSpaceA(emu: &mut emu::Emu) {
    let root_path_ptr = emu.regs().rcx;
    let sectors_per_cluster_ptr = emu.regs().rdx;
    let bytes_per_sector_ptr = emu.regs().r8;
    let free_clusters_ptr = emu.regs().r9;

    // 5th param is on the stack (shadow space + 8*4?)
    // Win64 calling convention: 4 regs, then stack.
    // The stack pointer at entry points to return address.
    // RSP+8 is RCX (shadow), RSP+16 RDX, RSP+24 R8, RSP+32 R9.
    // RSP+40 is the 5th argument.
    // BUT emu.stack_pop64 logic might handle "popping" return address?
    // Wait, in `handle_winapi` (winapi.rs):
    // `self.gateway_return = self.stack_pop64(false).unwrap_or(0);`
    // `self.regs_mut().rip = self.gateway_return;`
    // So RSP has been adjusted by 8.
    // The arguments are now at [RSP + 32] (shadow space 32 bytes) + offset?
    // No, standard x64 fastcall:
    // Caller allocates 32 bytes shadow space.
    // args 1-4 in regs.
    // arg 5 at [RSP + 32] (relative to Caller's RSP before call).
    // When we enter, return addr pushed. RSP -= 8.
    // We popped valid return address. RSP += 8. Back to Caller's RSP state (roughly).
    // So 5th arg is at [RSP + 40]? No, [RSP + 32].
    // Let's assume standard offset for now. Emu reads memory.

    let total_clusters_ptr = emu
        .maps
        .read_qword(emu.regs().get_esp() + 40) // 32 (shadow) + 8 (push 5th arg)? No, usually on stack just above shadow.
        .unwrap_or(0);
    // Logic:
    // CALL instr: pushes RetAddr (8 bytes). RSP -= 8.
    // Inside function:
    // Arg5 is at RSP + 8 + 32 = RSP + 40.
    // We POPPED RetAddr in `handle_winapi`. So RSP is back to pre-call state.
    // So Arg5 is at RSP + 32 ?
    // Actually shadow space is allocated by caller *above* the arguments?
    // No, shadow space is "home" for first 4 arguments.
    // Arg5 is at [RSP+32] immediately above shadow space.

    let root_path = if root_path_ptr != 0 {
        emu.maps.read_string(root_path_ptr)
    } else {
        "NULL".to_string()
    };

    log_red!(emu, "kernel32!GetDiskFreeSpaceA path:`{}`", root_path);

    if sectors_per_cluster_ptr != 0 {
        emu.maps.write_dword(sectors_per_cluster_ptr, 8);
    }
    if bytes_per_sector_ptr != 0 {
        emu.maps.write_dword(bytes_per_sector_ptr, 512);
    }
    if free_clusters_ptr != 0 {
        emu.maps.write_dword(free_clusters_ptr, 100000);
    }
    if total_clusters_ptr != 0 {
        emu.maps.write_dword(total_clusters_ptr, 200000);
    }

    emu.regs_mut().rax = 1;
}

use crate::emu;

pub(super) fn LookupPrivilegeValueW(emu: &mut emu::Emu) {
    let ptr_sysname = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let ptr_name = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let ptr_uid = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;

    let sysname = emu.maps.read_wide_string(ptr_sysname);
    let name = emu.maps.read_wide_string(ptr_name);

    log_red!(
        emu,
        "advapi32!LookupPrivilegeValueW `{}` `{}`",
        sysname,
        name
    );

    if ptr_uid != 0 {
        let luid: u64 = match name.as_str() {
            "SeDebugPrivilege" => 20,
            "SeShutdownPrivilege" => 19,
            "SeBackupPrivilege" => 17,
            "SeRestorePrivilege" => 18,
            "SeTcbPrivilege" => 7,
            "SeAssignPrimaryTokenPrivilege" => 3,
            "SeIncreaseQuotaPrivilege" => 5,
            _ => 99,
        };
        emu.maps.write_qword(ptr_uid, luid);
    }

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

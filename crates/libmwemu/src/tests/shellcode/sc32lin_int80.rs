use std::fs;

use crate::arch::OperatingSystem;
use crate::emu32;
use crate::syscall::linux::constants::syscalls::NR32_GETPID;

#[test]
fn raw_linux_file_dispatches_int80_without_windows_maps() {
    const CODE_BASE: u64 = 0x500000;
    const INT80: [u8; 2] = [0xcd, 0x80];
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("getpid.bin");
    fs::write(&path, INT80.repeat(2)).unwrap();

    for offset in [0, INT80.len() as u64] {
        let mut emu = emu32();
        emu.set_os(OperatingSystem::Linux);
        emu.set_base_address(CODE_BASE);
        emu.cfg.maps_folder = directory.path().to_str().unwrap().to_owned();
        if offset != 0 {
            emu.cfg.entry_point = CODE_BASE + offset;
        }
        emu.load_code(path.to_str().unwrap()).unwrap();
        assert_eq!(emu.pc(), CODE_BASE + offset);
        assert!(emu.maps.get_map_by_name("peb").is_none());

        emu.regs_mut().rax = NR32_GETPID;
        assert!(emu.step());
        assert_eq!(emu.regs().rax, emu.processes.pid);
        assert_eq!(emu.pc(), CODE_BASE + offset + INT80.len() as u64);
        assert!(emu.api_policy.last_outcome.is_none());
    }
}

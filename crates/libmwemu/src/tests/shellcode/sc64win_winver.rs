#[cfg(feature = "asset-tests")]
use crate::emu64;
use crate::tests::helpers;

/// Regression for shellcode loading under `--winver`.
///
/// Shellcode needs a placeholder host EXE (`loader.exe`) as the PEB's main
/// image. It is an mwemu support file, not a symbol-server artifact, so the
/// winver maps folder used to lack it and the load pointed the PEB ImageBase at
/// a missing file. `set_maps_from_winver` now seeds it from the bundled
/// `maps/windows/<arch>/` folder.
///
/// Run explicitly to exercise the symbol-server setup on a cold cache.
#[cfg(feature = "asset-tests")]
#[test]
#[ignore = "provisions Windows DLLs from the symbol server"]
pub fn sc64win_winver() {
    let sample = sample!("sc64win_metasploit.bin");
    helpers::setup();

    let mut emu = emu64();
    helpers::set_winver_maps(&mut emu, "win11");

    // The fix: loader.exe must be seeded into the winver maps folder.
    let loader = std::path::Path::new(&emu.cfg.maps_folder).join("loader.exe");
    assert!(
        loader.is_file(),
        "loader.exe must be seeded into the winver maps folder ({})",
        loader.display()
    );

    // End-to-end: the same metasploit shellcode the static-maps test uses must
    // load and run to the same point under winver maps.
    emu.load_code(&sample);
    emu.run(Some(0x3c00c8));
    emu.step();
    emu.run(Some(0x3c00c8));
    emu.step();
    emu.run(Some(0x3c00c8));
    emu.step();
    emu.run(Some(0x3c00c8));

    let stack = emu.regs().rsp;
    let sockaddr_ptr = emu.maps.read_qword(stack + 8).unwrap();
    let sockaddr = emu.maps.read_qword(sockaddr_ptr).unwrap();

    assert_eq!(sockaddr, 0x12c190a5c110002);
}

#[test]
fn empty_winver_cache_gets_the_bundled_loader_offline() {
    use crate::emu::winver::seed_loader_exe;

    for (is_x64, expected) in [(false, helpers::LOADER32), (true, helpers::LOADER64)] {
        let cache = tempfile::tempdir().unwrap();
        seed_loader_exe(cache.path(), is_x64);
        assert_eq!(
            std::fs::read(cache.path().join("loader.exe")).unwrap(),
            expected
        );
    }
}

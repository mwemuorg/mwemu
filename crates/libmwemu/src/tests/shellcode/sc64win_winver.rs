use crate::{tests::helpers, *};

/// Regression for shellcode loading under `--winver`.
///
/// Shellcode needs a placeholder host EXE (`loader.exe`) as the PEB's main
/// image. It is an mwemu support file, not a symbol-server artifact, so the
/// winver maps folder used to lack it and the load pointed the PEB ImageBase at
/// a missing file. `set_maps_from_winver` now seeds it from the bundled
/// `maps/windows/<arch>/` folder.
///
/// Gated on network availability: `set_winver_maps` returns `false` (skip) when
/// the symbol server or the seed NLS tables aren't reachable, keeping the suite
/// green offline.
#[test]
pub fn sc64win_winver() {
    helpers::setup();

    let mut emu = emu64();
    if !helpers::set_winver_maps(&mut emu, "win11") {
        return; // offline / no winver cache — skip
    }

    // The fix: loader.exe must be seeded into the winver maps folder.
    let loader = std::path::Path::new(&emu.cfg.maps_folder).join("loader.exe");
    assert!(
        loader.is_file(),
        "loader.exe must be seeded into the winver maps folder ({})",
        loader.display()
    );

    // End-to-end: the same metasploit shellcode the static-maps test uses must
    // load and run to the same point under winver maps.
    let sample = helpers::test_data_path("sc64win_metasploit.bin");
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

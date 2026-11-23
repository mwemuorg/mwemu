use crate::emu64;
use crate::tests::helpers;

#[test]
fn exe64win_enigma() {
    const STARTUP_INSTRUCTIONS: u64 = 5_000_000;

    helpers::setup();
    let sample = sample!("exe64win_enigma.bin");
    let mut emu = emu64();
    emu.cfg.maps_folder = win_maps!(64);
    emu.load_code(&sample).expect("load Enigma");
    emu.run_to(STARTUP_INSTRUCTIONS)
        .expect("run Enigma startup");
    assert!(emu.pos >= STARTUP_INSTRUCTIONS, "stopped at {}", emu.pos);
}

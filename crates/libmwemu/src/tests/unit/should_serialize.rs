use crate::{serialization::Serialization, tests::helpers, *};

#[test]
// test serialization
pub fn should_serialize() {
    helpers::setup();

    let handle = std::thread::Builder::new()
        .stack_size(1024 * 29055)
        .spawn(|| {
            // init
            let mut emu = emu64();

            // load maps
            emu.cfg.maps_folder = helpers::win64_maps_folder();

            // load binary
            emu.load_code(&helpers::test_data_path("exe64win_msgbox.bin"));

            // set registers
            emu.regs_mut().rdx = 0x1;

            // serialize
            let serialized = Serialization::serialize(&emu);

            // deserialize
            let emu: Emu = Serialization::deserialize(&serialized);
            // assert
            assert_eq!(emu.regs().rdx, 0x1);
        })
        .unwrap();

    handle.join().unwrap();
}

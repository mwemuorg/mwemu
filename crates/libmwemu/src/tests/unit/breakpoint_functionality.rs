use crate::tests::helpers;
use crate::*;

#[test]
// test breakpoint functionality, improve this with a running sample.
pub fn breakpoint_functionality() {
    helpers::setup();

    let mut bp = crate::debug::breakpoint::Breakpoint::new();

    // Test initial state
    assert_eq!(bp.get_bp(), 0);

    // Test basic breakpoint operations
    bp.set_bp(0x401000);
    assert_eq!(bp.get_bp(), 0x401000);

    // Test memory breakpoints
    bp.set_mem_read(0x402000);
    bp.set_mem_write(0x403000);

    // Test instruction breakpoints
    bp.set_instruction(100);

    // Test clearing breakpoints
    bp.clear_bp();
    assert_eq!(bp.get_bp(), 0);

    // Test multiple breakpoint operations
    bp.set_bp(0x500000);
    bp.set_mem_read(0x600000);
    bp.set_mem_write(0x700000);
    bp.set_instruction(200);

    assert_eq!(bp.get_bp(), 0); // only one type of bt at once, the setters clear all the
    // breakpointts.

    bp.clear_bp();
    assert_eq!(bp.get_bp(), 0);

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();

    emu.load_code(&helpers::test_data_path("exe64win_msgbox.bin"));
    assert!(!emu.maps.is_allocated(0));
    emu.bp.clear_bp();
    emu.bp.add_bp(0x1400011d6);
    emu.run(None);
    assert!(!emu.maps.is_allocated(0));
    assert_eq!(emu.pos, 4);

    /*
        emu.bp.set_mem_write(0x329f70);
        emu.run(None);
        assert_eq!(emu.pos, 15);

    14 0x1400010df: mov   [rbp-10h],rdi
    mem_trace: pos = 14 rip = 1400010df op = write bits = 64 address = 0x329f70 value = 0x1400011df name = 'stack'

        */

    emu.bp.clear_bp();
    emu.bp.add_bp_instruction(100);
    assert_eq!(emu.bp.instruction, [100]);
    emu.run(None);
    assert_eq!(emu.pos, 100);

    /* is not matching
        emu.bp.set_mem_read(0x329eb8);
        emu.run(None);
        assert_eq!(emu.pos, 102);

    mem_trace: pos = 102 rip = 1400010c4 op = read bits = 64 address = 0x329eb8 value = 0xc name = 'stack'
    mem_trace: pos = 102 rip = 1400010c4 op = write bits = 64 address = 0x329eb8 value = 0xc name = 'register'
    102 0x1400010c4: pop   rcx ;0xc
        */
}

use std::cell::Cell;
use std::rc::Rc;

use crate::exception::types::ExceptionType;
use crate::{emu32, tests::helpers};

const CHECKPOINT: u64 = 30_862_819;

#[test]
fn sc32win_donut() {
    helpers::setup();
    let sample = sample!("sc32win_donut.bin");
    let maps = win_maps!(32);

    let mut emu = emu32();
    emu.cfg.maps_folder = maps;
    emu.load_code(&sample).expect("load Donut fixture");
    let code_end = emu
        .maps
        .get_map_by_name("code")
        .expect("raw code map")
        .get_bottom();

    let last_exception = Rc::new(Cell::new(None));
    let observed = Rc::clone(&last_exception);
    emu.set_hook_on_exception(move |_, pc, kind| {
        observed.set(Some((pc, kind)));
        true
    });

    match emu.run_to(CHECKPOINT) {
        Ok(_) => {
            assert_eq!(emu.pos, CHECKPOINT);
            assert_eq!(emu.regs().get_eax(), 0xF5B24B1D);
            assert_eq!(emu.regs().get_ebx(), 0x12);
        }
        Err(error) => {
            // The truncated fixture can reach the final zero byte and fault fetching the next byte.
            let fault_pc = code_end - 1;
            assert_eq!(
                last_exception.get(),
                Some((fault_pc, ExceptionType::SettingRipToNonMappedAddr)),
                "{error}"
            );
            assert_eq!(emu.pc(), fault_pc, "{error}");
            assert!(emu.process_terminated, "{error}");
            assert_eq!(emu.maps.read_byte(fault_pc), Some(0));
            assert!(!emu.maps.is_mapped(code_end));
        }
    }
}

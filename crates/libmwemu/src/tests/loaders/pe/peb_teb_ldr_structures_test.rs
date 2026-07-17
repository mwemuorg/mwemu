use crate::windows::structures;
use crate::{tests::helpers, *};

#[test]
// peb/teb/ldr basic tests
pub fn peb_teb_ldr_structures_test() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = helpers::win32_maps_folder();
    emu.load_code(&sample!("exe32win_minecraft.bin"));

    let peb = emu.maps.get_mem("peb");
    let peb_addr = peb.get_base();
    assert!(peb_addr > 0x1000);
    assert!(emu.maps.is_allocated(peb_addr));
    let teb = emu.maps.get_mem("teb");
    let teb_addr = teb.get_base();
    assert!(teb_addr > 0x1000);
    assert!(emu.maps.is_allocated(teb_addr));
    let ldr = emu.maps.get_mem("ldr");
    let ldr_addr = ldr.get_base();
    assert!(ldr_addr > 0x1000);
    assert!(emu.maps.is_allocated(ldr_addr));

    let peb_struct = structures::PEB::load(peb_addr, &mut emu.maps);
    let teb_struct = structures::TEB::load(teb_addr, &mut emu.maps);
    let ldr_struct = structures::PebLdrData::load(ldr_addr, &mut emu.maps);

    assert_eq!(
        ldr_struct.in_load_order_module_list.flink,
        ldr_struct.in_memory_order_module_list.flink - 0x8
    );
    assert_eq!(
        ldr_struct.in_initialization_order_module_list.flink,
        ldr_struct.in_memory_order_module_list.flink + 0x8
    );
    assert_eq!(ldr_addr, peb_struct.ldr as u64);

    let mut ldr_entry = structures::LdrDataTableEntry::load(
        ldr_struct.in_load_order_module_list.flink as u64,
        &mut emu.maps,
    );
    let ntdll_addr = emu.maps.get_mem("ntdll.pe").get_base();

    assert_eq!(peb_struct.image_base_addr, ntdll_addr as u32);
    assert_eq!(peb_struct.ldr, ldr_addr as u32);
    assert_eq!(peb_struct.being_debugged, 0);

    assert!(teb_struct.process_id > 0);
    assert!(teb_struct.thread_id > 0);

    assert_eq!(teb_struct.process_environment_block, peb_addr as u32);
    assert_eq!(teb_struct.last_error_value, 0);
    //assert!(teb_struct.environment_pointer > 0);

    let main_pe_w = emu.maps.get_addr_name(ldr_entry.dll_base as u64);
    assert!(main_pe_w.is_some());
    let main_pe = main_pe_w.unwrap();
    assert_eq!(main_pe, "exe32win_minecraft.pe");

    assert_eq!(
        ldr_entry.in_memory_order_links.flink,
        ldr_entry.in_load_order_links.flink + 0x8
    );
    assert_eq!(
        ldr_entry.in_initialization_order_links.flink,
        ldr_entry.in_memory_order_links.flink + 0x8
    );

    assert_eq!(
        ldr_entry.in_memory_order_links.blink,
        ldr_entry.in_load_order_links.blink + 0x8
    );
    assert_eq!(
        ldr_entry.in_initialization_order_links.blink,
        ldr_entry.in_memory_order_links.blink + 0x8
    );

    let sample_w = emu.maps.get_addr_name(ldr_entry.dll_base as u64);
    assert!(sample_w.is_some());
    let sample = sample_w.unwrap();
    assert_eq!(sample, "exe32win_minecraft.pe");

    // The core libraries follow the executable in loader initialization order.
    for expected in ["ntdll.dll", "kernel32.dll", "kernelbase.dll"] {
        ldr_entry = structures::LdrDataTableEntry::load(
            ldr_entry.in_load_order_links.flink as u64,
            &mut emu.maps,
        );

        assert_eq!(
            ldr_entry.in_memory_order_links.flink,
            ldr_entry.in_load_order_links.flink + 0x8
        );
        assert_eq!(
            ldr_entry.in_initialization_order_links.flink,
            ldr_entry.in_memory_order_links.flink + 0x8
        );
        assert_eq!(
            ldr_entry.in_memory_order_links.blink,
            ldr_entry.in_load_order_links.blink + 0x8
        );
        assert_eq!(
            ldr_entry.in_initialization_order_links.blink,
            ldr_entry.in_memory_order_links.blink + 0x8
        );
        assert_eq!(
            emu.maps
                .read_wide_string(ldr_entry.base_dll_name.buffer as u64),
            expected
        );
    }

    // Dependencies are appended after the core prefix; retain the original
    // netapi32 assertions without requiring it to occupy a fixed position.
    let first_entry = ldr_struct.in_load_order_module_list.flink as u64;
    let mut found_netapi = false;
    for _ in 0..4096 {
        let name = emu
            .maps
            .read_wide_string(ldr_entry.base_dll_name.buffer as u64);
        if name == "netapi32.dll" {
            found_netapi = true;
            break;
        }
        let next = ldr_entry.in_load_order_links.flink as u64;
        if next == first_entry {
            break;
        }
        ldr_entry = structures::LdrDataTableEntry::load(next, &mut emu.maps);
    }
    assert!(
        found_netapi,
        "netapi32.dll should remain linked after core modules"
    );

    let sample_w = emu.maps.get_addr_name(ldr_entry.dll_base as u64);
    assert!(sample_w.is_some());
    let sample = sample_w.unwrap();
    assert_eq!(sample, "netapi32.pe");

    let ntdll_str_ptr = ldr_entry.base_dll_name.buffer as u64;
    assert!(ntdll_str_ptr > 0);
    let ntdll_str = emu.maps.read_wide_string(ntdll_str_ptr);
    assert_eq!(ntdll_str, "netapi32.dll");

    let ntdll_str_ptr = ldr_entry.full_dll_name.buffer as u64;
    assert!(ntdll_str_ptr > 0);
    let ntdll_str = emu.maps.read_wide_string(ntdll_str_ptr);
    assert_eq!(ntdll_str, "C:\\Windows\\System32\\netapi32.dll");

    // 64BITS //

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();
    emu.load_code(&sample!("exe64win_msgbox.bin"));

    let ntdll_addr = emu.maps.get_mem("ntdll.pe").get_base();

    let peb = emu.maps.get_mem("peb");
    let peb_addr = peb.get_base();
    assert!(peb_addr > 0x1000);
    assert!(emu.maps.is_allocated(peb_addr));
    let teb = emu.maps.get_mem("teb");
    let teb_addr = teb.get_base();
    assert!(teb_addr > 0x1000);
    assert!(emu.maps.is_allocated(teb_addr));
    let ldr = emu.maps.get_mem("ldr");
    let ldr_addr = ldr.get_base();
    assert!(ldr_addr > 0x1000);
    assert!(emu.maps.is_allocated(ldr_addr));

    let peb_struct = structures::PEB64::load(peb_addr, &mut emu.maps);
    let teb_struct = structures::TEB64::load(teb_addr, &mut emu.maps);

    assert_eq!(peb_struct.image_base_addr, ntdll_addr);
    assert_eq!(peb_struct.ldr, ldr_addr);
    assert_eq!(peb_struct.being_debugged, 0);

    assert!(teb_struct.process_id > 0);
    assert!(teb_struct.thread_id > 0);

    assert_eq!(teb_struct.process_environment_block, peb_addr);
    assert_eq!(teb_struct.last_error_value, 0);
    //assert!(teb_struct.environment_pointer > 0);

    let ldr_struct = structures::PebLdrData64::load(ldr_addr, &mut emu.maps);
    let entry_addr = ldr_struct.in_load_order_module_list.flink;
    assert!(entry_addr >= 0x1000);
    let mut ldr_entry = structures::LdrDataTableEntry64::load(entry_addr, &mut emu.maps);

    //let ntdll_addr = emu.maps.get_mem("ntdll.pe").get_base();

    assert_eq!(
        ldr_entry.in_memory_order_links.flink,
        ldr_entry.in_load_order_links.flink + 0x10
    );
    assert_eq!(
        ldr_entry.in_initialization_order_links.flink,
        ldr_entry.in_memory_order_links.flink + 0x10
    );

    assert_eq!(
        ldr_entry.in_memory_order_links.blink,
        ldr_entry.in_load_order_links.blink + 0x10
    );
    assert_eq!(
        ldr_entry.in_initialization_order_links.blink,
        ldr_entry.in_memory_order_links.blink + 0x10
    );

    let sample_w = emu.maps.get_addr_name(ldr_entry.dll_base);
    assert!(sample_w.is_some());
    let sample = sample_w.unwrap();
    assert_eq!(sample, "exe64win_msgbox.pe");

    // follow to next flink (ntdll)
    ldr_entry =
        structures::LdrDataTableEntry64::load(ldr_entry.in_load_order_links.flink, &mut emu.maps);

    assert_eq!(
        ldr_entry.in_memory_order_links.flink,
        ldr_entry.in_load_order_links.flink + 0x10
    );
    assert_eq!(
        ldr_entry.in_initialization_order_links.flink,
        ldr_entry.in_memory_order_links.flink + 0x10
    );

    let module = emu.maps.read_wide_string(ldr_entry.base_dll_name.buffer);
    assert_eq!(module, "ntdll.dll");

    ldr_entry =
        structures::LdrDataTableEntry64::load(ldr_entry.in_load_order_links.flink, &mut emu.maps);
    assert_eq!(
        emu.maps.read_wide_string(ldr_entry.base_dll_name.buffer),
        "kernel32.dll"
    );

    ldr_entry =
        structures::LdrDataTableEntry64::load(ldr_entry.in_load_order_links.flink, &mut emu.maps);
    assert_eq!(
        emu.maps.read_wide_string(ldr_entry.base_dll_name.buffer),
        "kernelbase.dll"
    );

    assert_eq!(
        ldr_entry.in_memory_order_links.blink,
        ldr_entry.in_load_order_links.blink + 0x10
    );
    assert_eq!(
        ldr_entry.in_initialization_order_links.blink,
        ldr_entry.in_memory_order_links.blink + 0x10
    );

    let sample_w = emu.maps.get_addr_name(ldr_entry.dll_base);
    assert!(sample_w.is_some());
    let sample = sample_w.unwrap();
    assert_eq!(sample, "kernelbase.pe");

    let ntdll_str_ptr = ldr_entry.base_dll_name.buffer as u64;
    assert!(ntdll_str_ptr > 0);
    let ntdll_str = emu.maps.read_wide_string(ntdll_str_ptr);
    assert_eq!(ntdll_str, "kernelbase.dll");

    let ntdll_str_ptr = ldr_entry.full_dll_name.buffer as u64;
    assert!(ntdll_str_ptr > 0);
    let ntdll_str = emu.maps.read_wide_string(ntdll_str_ptr);
    assert_eq!(ntdll_str, "C:\\Windows\\System32\\kernelbase.dll");
}

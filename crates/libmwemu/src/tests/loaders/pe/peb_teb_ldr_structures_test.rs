use crate::maps::mem64::Permission;
#[cfg(feature = "asset-tests")]
use crate::tests::helpers;
use crate::windows::peb::{peb32, peb64};
use crate::windows::structures;
use crate::*;

#[test]
fn peb_round_trip_preserves_guest_layout_on_both_architectures() {
    const INPUT: u64 = 0x10000;
    const OUTPUT: u64 = 0x20000;
    const POST_INIT32: u64 = 0x14c;
    const SESSION32: u64 = 0x1d4;
    const POST_INIT64: u64 = 0x230;
    const TLS_BITMAP64: u64 = 0x238;
    const TLS_BITS64: u64 = 0x240;
    const SESSION64: u64 = 0x2c0;
    const DECOMMIT_TOTAL64: u64 = 0xd8;
    const DECOMMIT_BLOCK64: u64 = 0xe0;

    for is64 in [false, true] {
        let mut emu = if is64 { emu64() } else { emu32() };
        let size = if is64 {
            structures::Peb64::size()
        } else {
            structures::Peb::size()
        };
        for (name, address) in [("peb_input", INPUT), ("peb_output", OUTPUT)] {
            emu.maps
                .create_map(name, address, size as u64, Permission::READ_WRITE)
                .unwrap();
        }
        emu.maps.memset(OUTPUT, 0xcc, size);
        if is64 {
            let fields = [
                (DECOMMIT_TOTAL64, 0x1000),
                (DECOMMIT_BLOCK64, 0x2000),
                (POST_INIT64, 0x1234_5000),
                (TLS_BITMAP64, 0x7777_8000),
            ];
            for (offset, value) in fields {
                emu.maps.write_qword(INPUT + offset, value);
            }
            emu.maps.write_dword(INPUT + TLS_BITS64, 1);
            emu.maps.write_dword(INPUT + SESSION64 - 4, 0x8000_0000);
            emu.maps.write_dword(INPUT + SESSION64, 7);
            let peb = structures::Peb64::load(INPUT, &emu.maps);
            peb.save(emu.maps.get_mem_mut("peb_output"));
            for (offset, value) in fields {
                assert_eq!(emu.maps.read_qword(OUTPUT + offset), Some(value));
            }
            for (offset, value) in [
                (TLS_BITS64, 1),
                (SESSION64 - 4, 0x8000_0000),
                (SESSION64, 7),
                (SESSION64 + 4, 0xcccc_cccc),
            ] {
                assert_eq!(emu.maps.read_dword(OUTPUT + offset), Some(value));
            }
        } else {
            emu.maps.write_dword(INPUT + POST_INIT32, 0x1234_5000);
            emu.maps.write_dword(INPUT + SESSION32, 7);
            let peb = structures::Peb::load(INPUT, &emu.maps);
            peb.save(emu.maps.get_mem_mut("peb_output"));
            assert_eq!(emu.maps.read_dword(OUTPUT + POST_INIT32), Some(0x1234_5000));
            assert_eq!(emu.maps.read_dword(OUTPUT + SESSION32), Some(7));
            assert_eq!(emu.maps.read_dword(OUTPUT + 0x38), Some(0xcccc_cccc));
        }
    }
}

#[test]
fn loader_entries_preserve_guest_image_and_dll_paths() {
    let executable = "tést.exe";
    let process_path = r"C:\tést.exe";
    let dll_path = r"C:\Windows\System32\ntdll.dll";
    for is64 in [false, true] {
        let mut emu = if is64 { emu64() } else { emu32() };
        emu.cfg.exe_name = executable.into();
        let (process, dll) = if is64 {
            let ldr = peb64::init_ldr(&mut emu);
            let entry = structures::PebLdrData64::load(ldr, &emu.maps)
                .in_load_order_module_list
                .flink;
            (
                entry,
                peb64::create_ldr_entry(&mut emu, 0, 0, "ntdll.dll", 0, 0),
            )
        } else {
            let ldr = peb32::init_ldr(&mut emu);
            let entry = structures::PebLdrData::load(ldr, &emu.maps)
                .in_load_order_module_list
                .flink;
            (
                u64::from(entry),
                peb32::create_ldr_entry(&mut emu, 0, 0, "ntdll.dll", 0, 0),
            )
        };
        for (entry, full, base) in [
            (process, process_path, executable),
            (dll, dll_path, "ntdll.dll"),
        ] {
            let names = if is64 {
                let entry = structures::LdrDataTableEntry64::load(entry, &emu.maps);
                [entry.full_dll_name, entry.base_dll_name]
                    .map(|s| (s.buffer, s.length, s.maximum_length))
            } else {
                let entry = structures::LdrDataTableEntry::load(entry, &emu.maps);
                [entry.full_dll_name, entry.base_dll_name]
                    .map(|s| (u64::from(s.buffer), s.length, s.maximum_length))
            };
            for ((buffer, length, capacity), expected) in names.into_iter().zip([full, base]) {
                assert_eq!(emu.maps.read_wide_string(buffer), expected);
                assert_eq!(length as usize, expected.encode_utf16().count() * 2);
                assert_eq!(capacity, length + 2);
            }
        }
    }
}

#[cfg(feature = "asset-tests")]
#[test]
// peb/teb/ldr basic tests
pub fn peb_teb_ldr_structures_test() {
    helpers::setup();

    assert_eq!(structures::TEB::size(), 0x1038);
    assert_eq!(structures::TEB::map_size(), 0x2000);
    assert_eq!(structures::TEB64::size(), 0x1878);
    assert_eq!(structures::TEB64::map_size(), 0x2000);

    let mut emu = emu32();
    emu.cfg.maps_folder = helpers::fixture_maps32();
    emu.load_code(&sample!("exe32win_minecraft.bin"));

    let peb = emu.maps.get_mem("peb");
    let peb_addr = peb.get_base();
    assert!(peb_addr > 0x1000);
    assert!(emu.maps.is_allocated(peb_addr));
    let teb = emu.maps.get_mem("teb");
    let teb_addr = teb.get_base();
    assert!(teb_addr > 0x1000);
    assert!(emu.maps.is_allocated(teb_addr));
    assert_eq!(teb.size(), structures::TEB::map_size());
    for offset in 0x1034..0x1038 {
        assert!(emu.maps.write_byte(teb_addr + offset, offset as u8));
        assert_eq!(emu.maps.read_byte(teb_addr + offset), Some(offset as u8));
    }
    let ldr = emu.maps.get_mem("ldr");
    let ldr_addr = ldr.get_base();
    assert!(ldr_addr > 0x1000);
    assert!(emu.maps.is_allocated(ldr_addr));

    let peb_struct = structures::Peb::load(peb_addr, &emu.maps);
    let mut teb_struct = structures::TEB::load(teb_addr, &emu.maps);
    let ldr_struct = structures::PebLdrData::load(ldr_addr, &emu.maps);

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
        &emu.maps,
    );
    let main_base = emu.maps.get_mem("exe32win_minecraft.pe").get_base();

    assert_eq!(peb_struct.image_base_addr, main_base as u32);
    assert_eq!(peb_struct.ldr, ldr_addr as u32);
    assert_eq!(peb_struct.being_debugged, 0);

    assert!(teb_struct.process_id > 0);
    assert!(teb_struct.thread_id > 0);

    assert_eq!(teb_struct.process_environment_block, peb_addr as u32);
    assert_eq!(teb_struct.last_error_value, 0);
    teb_struct.process_id = 0x1122_3344;
    teb_struct.thread_id = 0x5566_7788;
    teb_struct.current_locale = 0x409;
    teb_struct.exception_code = 0xc001_cafe;
    teb_struct.activation_context_stack_pointer = 0xaabb_ccdd;
    teb_struct.user32_reserved[25] = 0x2525_2525;
    teb_struct.user_reserved[4] = 0x4545_4545;
    teb_struct.save(emu.maps.get_mem_mut("teb"));
    for offset in 0x1034..0x1038 {
        assert_eq!(emu.maps.read_byte(teb_addr + offset), Some(offset as u8));
    }

    let teb_round_trip = structures::TEB::load(teb_addr, &emu.maps);
    assert_eq!(teb_round_trip.process_id, 0x1122_3344);
    assert_eq!(teb_round_trip.thread_id, 0x5566_7788);
    assert_eq!(teb_round_trip.process_environment_block, peb_addr as u32);
    assert_eq!(teb_round_trip.current_locale, 0x409);
    assert_eq!(teb_round_trip.exception_code, 0xc001_cafe);
    assert_eq!(teb_round_trip.activation_context_stack_pointer, 0xaabb_ccdd);
    assert_eq!(teb_round_trip.user32_reserved[25], 0x2525_2525);
    assert_eq!(teb_round_trip.user_reserved[4], 0x4545_4545);
    let teb_map_round_trip = structures::TEB::load_map(teb_addr, emu.maps.get_mem("teb"));
    assert_eq!(teb_map_round_trip.process_id, 0x1122_3344);
    assert_eq!(teb_map_round_trip.user_reserved[4], 0x4545_4545);
    assert_eq!(
        emu.maps
            .read_dword(teb_addr + structures::TEB::CLIENT_ID_PROCESS_ID_OFFSET),
        Some(0x1122_3344)
    );
    assert_eq!(
        emu.maps
            .read_dword(teb_addr + structures::TEB::CLIENT_ID_THREAD_ID_OFFSET),
        Some(0x5566_7788)
    );
    assert_eq!(
        emu.maps
            .read_dword(teb_addr + structures::TEB::PROCESS_ENVIRONMENT_BLOCK_OFFSET),
        Some(peb_addr as u32)
    );
    assert_eq!(
        emu.maps
            .read_dword(teb_addr + structures::TEB::CURRENT_LOCALE_OFFSET),
        Some(0x409)
    );
    assert_eq!(
        emu.maps
            .read_dword(teb_addr + structures::TEB::EXCEPTION_CODE_OFFSET),
        Some(0xc001_cafe)
    );
    assert_eq!(
        emu.maps
            .read_dword(teb_addr + structures::TEB::ACTIVATION_CONTEXT_STACK_POINTER_OFFSET,),
        Some(0xaabb_ccdd)
    );
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
            &emu.maps,
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
        ldr_entry = structures::LdrDataTableEntry::load(next, &emu.maps);
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
    emu.cfg.maps_folder = helpers::fixture_maps64();
    emu.load_code(&sample!("exe64win_msgbox.bin"));

    let peb = emu.maps.get_mem("peb");
    let peb_addr = peb.get_base();
    assert!(peb_addr > 0x1000);
    assert!(emu.maps.is_allocated(peb_addr));
    let teb = emu.maps.get_mem("teb");
    let teb_addr = teb.get_base();
    assert!(teb_addr > 0x1000);
    assert!(emu.maps.is_allocated(teb_addr));
    assert_eq!(teb.size(), structures::TEB64::map_size());
    assert!(
        emu.maps
            .write_byte(teb_addr + structures::TEB64::size() as u64 - 1, 0xa5)
    );
    assert_eq!(
        emu.maps
            .read_byte(teb_addr + structures::TEB64::size() as u64 - 1),
        Some(0xa5)
    );
    let ldr = emu.maps.get_mem("ldr");
    let ldr_addr = ldr.get_base();
    assert!(ldr_addr > 0x1000);
    assert!(emu.maps.is_allocated(ldr_addr));

    let peb_struct = structures::Peb64::load(peb_addr, &emu.maps);
    let mut teb_struct = structures::TEB64::load(teb_addr, &emu.maps);

    let main_base64 = emu.maps.get_mem("exe64win_msgbox.pe").get_base();
    assert_eq!(peb_struct.image_base_addr, main_base64);
    assert_eq!(peb_struct.ldr, ldr_addr);
    assert_eq!(peb_struct.being_debugged, 0);

    assert!(teb_struct.process_id > 0);
    assert!(teb_struct.thread_id > 0);

    assert_eq!(teb_struct.process_environment_block, peb_addr);
    assert_eq!(teb_struct.last_error_value, 0);
    teb_struct.process_id = 0x1122_3344_5566_7788;
    teb_struct.thread_id = 0x8877_6655_4433_2211;
    teb_struct.current_locale = 0x1234;
    teb_struct.fp_software_status_register = 0x5678;
    teb_struct.exception_code = 0xc001_cafe;
    teb_struct.activation_context_stack_pointer = 0x1234_5678_9abc_def0;
    teb_struct.user32_reserved[0] = 0x1111_1111;
    teb_struct.user32_reserved[25] = 0x2525_2525;
    teb_struct.user_reserved[0] = 0x3333_3333;
    teb_struct.user_reserved[4] = 0x4545_4545;
    teb_struct.wow32_reserved = 0x1122_3344_5566_7788;
    teb_struct.save(emu.maps.get_mem_mut("teb"));

    let teb_round_trip = structures::TEB64::load(teb_addr, &emu.maps);
    assert_eq!(teb_round_trip.process_id, 0x1122_3344_5566_7788);
    assert_eq!(teb_round_trip.thread_id, 0x8877_6655_4433_2211);
    assert_eq!(teb_round_trip.current_locale, 0x1234);
    assert_eq!(teb_round_trip.fp_software_status_register, 0x5678);
    assert_eq!(teb_round_trip.exception_code, 0xc001_cafe);
    assert_eq!(
        teb_round_trip.activation_context_stack_pointer,
        0x1234_5678_9abc_def0
    );
    assert_eq!(teb_round_trip.user32_reserved[0], 0x1111_1111);
    assert_eq!(teb_round_trip.user32_reserved[25], 0x2525_2525);
    assert_eq!(teb_round_trip.user_reserved[0], 0x3333_3333);
    assert_eq!(teb_round_trip.user_reserved[4], 0x4545_4545);
    let teb_map_round_trip = structures::TEB64::load_map(teb_addr, emu.maps.get_mem("teb"));
    assert_eq!(teb_map_round_trip.process_id, 0x1122_3344_5566_7788);
    assert_eq!(teb_map_round_trip.user_reserved[4], 0x4545_4545);
    assert_eq!(
        emu.maps
            .read_qword(teb_addr + structures::TEB64::WOW32_RESERVED_OFFSET),
        Some(0x1122_3344_5566_7788)
    );
    assert_eq!(
        emu.maps
            .read_dword(teb_addr + structures::TEB64::CURRENT_LOCALE_OFFSET),
        Some(0x1234)
    );
    assert_eq!(
        emu.maps
            .read_dword(teb_addr + structures::TEB64::FP_SOFTWARE_STATUS_REGISTER_OFFSET,),
        Some(0x5678)
    );
    assert_eq!(
        emu.maps
            .read_dword(teb_addr + structures::TEB64::EXCEPTION_CODE_OFFSET,),
        Some(0xc001_cafe)
    );
    assert_eq!(
        emu.maps
            .read_qword(teb_addr + structures::TEB64::ACTIVATION_CONTEXT_STACK_POINTER_OFFSET,),
        Some(0x1234_5678_9abc_def0)
    );
    //assert!(teb_struct.environment_pointer > 0);

    let ldr_struct = structures::PebLdrData64::load(ldr_addr, &emu.maps);
    let entry_addr = ldr_struct.in_load_order_module_list.flink;
    assert!(entry_addr >= 0x1000);
    let mut ldr_entry = structures::LdrDataTableEntry64::load(entry_addr, &emu.maps);

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
        structures::LdrDataTableEntry64::load(ldr_entry.in_load_order_links.flink, &emu.maps);

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
        structures::LdrDataTableEntry64::load(ldr_entry.in_load_order_links.flink, &emu.maps);
    assert_eq!(
        emu.maps.read_wide_string(ldr_entry.base_dll_name.buffer),
        "kernel32.dll"
    );

    ldr_entry =
        structures::LdrDataTableEntry64::load(ldr_entry.in_load_order_links.flink, &emu.maps);
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

    let ntdll_str_ptr = ldr_entry.base_dll_name.buffer;
    assert!(ntdll_str_ptr > 0);
    let ntdll_str = emu.maps.read_wide_string(ntdll_str_ptr);
    assert_eq!(ntdll_str, "kernelbase.dll");

    let ntdll_str_ptr = ldr_entry.full_dll_name.buffer;
    assert!(ntdll_str_ptr > 0);
    let ntdll_str = emu.maps.read_wide_string(ntdll_str_ptr);
    assert_eq!(ntdll_str, "C:\\Windows\\System32\\kernelbase.dll");
}

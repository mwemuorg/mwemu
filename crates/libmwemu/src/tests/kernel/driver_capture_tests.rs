//! `*_register_driver` capture: the reachability bridge.
//!
//! When a `.ko`'s `init` hands the kernel its driver ops struct, mwemu walks
//! the struct and resolves the `.probe` callback and the `id_table` by *where
//! the pointers land* — text vs. data of the module image — rather than by a
//! hard-coded, per-bus, per-kernel-version offset. These tests fabricate a
//! module image and a driver struct so the resolver is exercised without a real
//! module on disk.

use crate::emu64;
use crate::kernel::KernelOs;
use crate::maps::mem64::Permission;
use rs_header::elf::Perm;
use rs_header::elf::relocatable::{RelSection, RelSymbol};

const MODULE_BASE: u64 = 0xffffffffc0000000; // Linux layout module_base.
const TEXT: u64 = MODULE_BASE; //            [BASE .. BASE+0x1000)  r-x
const DATA: u64 = MODULE_BASE + 0x1000; //   [+0x1000 .. +0x3000)   r--

/// Bring up a kernel env with a synthetic module: one executable text section
/// and one read-only data section, backed by a single writable map so the test
/// can plant pointers in either.
fn boot_with_module() -> crate::emu::Emu {
    let mut emu = emu64();
    emu.cfg.verbose = 0;
    emu.kernel_init(KernelOs::Linux);

    emu.maps
        .create_map("module.image", MODULE_BASE, 0x3000, Permission::READ_WRITE)
        .expect("map module image");

    let kernel = emu.kernel.as_mut().expect("kernel env");
    kernel.module.base = MODULE_BASE;
    kernel.module.size = 0x3000;
    kernel.module.sections = vec![
        RelSection {
            index: 1,
            name: ".text".to_string(),
            addr: TEXT,
            size: 0x1000,
            perm: Perm::from_flags(true, false, true),
        },
        RelSection {
            index: 2,
            name: ".rodata".to_string(),
            addr: DATA,
            size: 0x2000,
            perm: Perm::from_flags(true, false, false),
        },
    ];
    emu
}

/// Lay down a driver ops struct at `at`: { name, probe, id_table } qwords, with
/// the probe pointing into text and name/id_table into data. Returns
/// (struct_ptr, probe_addr, id_table_addr).
fn plant_driver(emu: &mut crate::emu::Emu, at: u64) -> (u64, u64, u64) {
    let probe = TEXT + 0x120;
    let name = DATA + 0x40;
    let id_table = DATA + 0x400;

    emu.kernel.as_mut().unwrap().module.symbols.push(RelSymbol {
        name: "tdrv_probe".to_string(),
        addr: probe,
        size: 0x80,
        is_func: true,
        is_global: false,
    });

    emu.maps.write_string(name, "tdrv");
    // A non-stringy id_table head so it is not mistaken for the name.
    emu.maps.write_qword(id_table, 0xdeadbeefcafe0000);

    // struct: off0 name, off8 probe, off16 id_table.
    emu.maps.write_qword(at, name);
    emu.maps.write_qword(at + 8, probe);
    emu.maps.write_qword(at + 16, id_table);
    (at, probe, id_table)
}

#[test]
fn captures_probe_and_id_table_from_driver_struct() {
    let mut emu = boot_with_module();
    let struct_ptr = DATA + 0x800;
    let (_, probe, id_table) = plant_driver(&mut emu, struct_ptr);

    let resolved = emu.kernel_register_driver("usb", struct_ptr);
    assert_eq!(resolved, probe, "probe resolved to the text pointer");

    let drivers = emu.kernel_registered_drivers();
    assert_eq!(drivers.len(), 1);
    let d = &drivers[0];
    assert_eq!(d.bus, "usb");
    assert_eq!(d.struct_ptr, struct_ptr);
    assert_eq!(d.probe, probe);
    assert_eq!(d.probe_name, "tdrv_probe");
    assert_eq!(d.id_table, id_table);
    assert_eq!(d.name, "tdrv");
}

#[test]
fn probe_is_the_first_text_pointer_not_a_later_one() {
    // A struct whose first qword is a *data* pointer and whose probe sits later
    // must still resolve — mirrors struct pci_driver (list_head + name before
    // probe).
    let mut emu = boot_with_module();
    let struct_ptr = DATA + 0x800;
    let probe = TEXT + 0x300;
    emu.kernel.as_mut().unwrap().module.symbols.push(RelSymbol {
        name: "pci_probe".to_string(),
        addr: probe,
        size: 0x40,
        is_func: true,
        is_global: false,
    });
    // list_head node (two data-ish pointers), then name, id_table, then probe.
    emu.maps.write_qword(struct_ptr, DATA + 0x10);
    emu.maps.write_qword(struct_ptr + 8, DATA + 0x20);
    emu.maps.write_string(DATA + 0x30, "pcidrv");
    emu.maps.write_qword(struct_ptr + 16, DATA + 0x30); // name
    emu.maps.write_qword(struct_ptr + 24, DATA + 0x500); // id_table
    emu.maps.write_qword(DATA + 0x500, 0x8086_1234_0000_0000);
    emu.maps.write_qword(struct_ptr + 32, probe);

    let resolved = emu.kernel_register_driver("pci", struct_ptr);
    assert_eq!(resolved, probe);
    let d = emu.kernel_registered_drivers();
    assert_eq!(d[0].probe_name, "pci_probe");
    assert_ne!(d[0].id_table, 0, "an id_table pointer was captured");
}

#[test]
fn no_module_pointers_yields_no_probe() {
    // A struct full of kernel/heap pointers (nothing into the module) resolves
    // to no probe rather than a false one.
    let mut emu = boot_with_module();
    let struct_ptr = DATA + 0x800;
    for i in 0..8u64 {
        // Kernel data-region pointers, outside the module image.
        emu.maps
            .write_qword(struct_ptr + i * 8, 0xffffffff82000000 + i * 8);
    }
    let resolved = emu.kernel_register_driver("platform", struct_ptr);
    assert_eq!(resolved, 0);
    let d = emu.kernel_registered_drivers();
    assert_eq!(d[0].probe, 0);
    assert_eq!(d[0].id_table, 0);
}

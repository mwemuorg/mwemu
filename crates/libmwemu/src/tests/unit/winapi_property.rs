use itertools::iproduct;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::Ordering;
use std::thread;

use rs_header::pe::export_index::{ExportIndexData, ExportTarget, NamedExport};

use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::tests::helpers::{register_export_module, setup};
use crate::winapi::{winapi32, winapi64};
use crate::{emu32, emu64};

const MODULE64: u64 = 0x170000000;
const MODULE32: u64 = 0x70000000;
const SENTINEL: u64 = 0xDEAD_C0DE_CAFE_BABE;

struct DllDef {
    name: &'static str,
    section: &'static str,
    sources: &'static [&'static str],
}

impl DllDef {
    fn api_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self
            .sources
            .iter()
            .flat_map(|source| extract_api_names(source))
            .collect();
        let common_sources: &[&str] = match self.name {
            "kernel32" | "kernelbase" => &[
                include_str!("../../api/windows/common/files.rs"),
                include_str!("../../api/windows/common/handles.rs"),
                include_str!("../../api/windows/common/modules.rs"),
                include_str!("../../api/windows/common/strings.rs"),
            ],
            "user32" => &[include_str!("../../api/windows/common/strings.rs")],
            _ => &[],
        };
        for source in common_sources {
            names.extend(extract_api_names(source));
        }
        names.sort();
        names.dedup();
        names
    }
}

const DLLS_64: &[DllDef] = &[
    DllDef {
        name: "ucrtbase",
        section: "ucrtbase.text",
        sources: &[
            include_str!("../../api/windows/winapi64/wincrt/mod.rs"),
            include_str!("../../api/windows/winapi64/wincrt/runtime/mod.rs"),
            include_str!("../../api/windows/winapi64/wincrt/memory/mod.rs"),
            include_str!("../../api/windows/winapi64/wincrt/io/mod.rs"),
            include_str!("../../api/windows/winapi64/wincrt/strings/mod.rs"),
            include_str!("../../api/windows/winapi64/wincrt/math/mod.rs"),
        ],
    },
    DllDef {
        name: "kernel32",
        section: "kernel32.text",
        sources: &[
            include_str!("../../api/windows/winapi64/kernel32/mod.rs"),
            include_str!("../../api/windows/winapi64/kernel32/appcompat/mod.rs"),
            include_str!("../../api/windows/winapi64/kernel32/console/mod.rs"),
            include_str!("../../api/windows/winapi64/kernel32/diagnostics/mod.rs"),
            include_str!("../../api/windows/winapi64/kernel32/filesystem/mod.rs"),
            include_str!("../../api/windows/winapi64/kernel32/library_loader/mod.rs"),
            include_str!("../../api/windows/winapi64/kernel32/memory/mod.rs"),
            include_str!("../../api/windows/winapi64/kernel32/objects/mod.rs"),
            include_str!("../../api/windows/winapi64/kernel32/process_thread/mod.rs"),
            include_str!("../../api/windows/winapi64/kernel32/registry_profile/mod.rs"),
            include_str!("../../api/windows/winapi64/kernel32/strings_nls/mod.rs"),
            include_str!("../../api/windows/winapi64/kernel32/sync/mod.rs"),
            include_str!("../../api/windows/winapi64/kernel32/system/mod.rs"),
            include_str!("../../api/windows/winapi64/kernel32/time_date/mod.rs"),
        ],
    },
    DllDef {
        name: "user32",
        section: "user32.text",
        sources: &[
            include_str!("../../api/windows/winapi64/user32/mod.rs"),
            include_str!("../../api/windows/winapi64/user32/clipboard/mod.rs"),
            include_str!("../../api/windows/winapi64/user32/dde/mod.rs"),
            include_str!("../../api/windows/winapi64/user32/dialogs/mod.rs"),
            include_str!("../../api/windows/winapi64/user32/graphics/mod.rs"),
            include_str!("../../api/windows/winapi64/user32/input/mod.rs"),
            include_str!("../../api/windows/winapi64/user32/menus/mod.rs"),
            include_str!("../../api/windows/winapi64/user32/messages/mod.rs"),
            include_str!("../../api/windows/winapi64/user32/strings_format/mod.rs"),
            include_str!("../../api/windows/winapi64/user32/system/mod.rs"),
            include_str!("../../api/windows/winapi64/user32/windowing/mod.rs"),
        ],
    },
    DllDef {
        name: "gdi32",
        section: "gdi32.text",
        sources: &[include_str!("../../api/windows/winapi64/gdi32/mod.rs")],
    },
    DllDef {
        name: "advapi32",
        section: "advapi32.text",
        sources: &[
            include_str!("../../api/windows/winapi64/advapi32/mod.rs"),
            include_str!("../../api/windows/winapi64/advapi32/accounts/mod.rs"),
            include_str!("../../api/windows/winapi64/advapi32/crypto/mod.rs"),
            include_str!("../../api/windows/winapi64/advapi32/registry_api/mod.rs"),
            include_str!("../../api/windows/winapi64/advapi32/security/mod.rs"),
            include_str!("../../api/windows/winapi64/advapi32/services/mod.rs"),
            include_str!("../../api/windows/winapi64/advapi32/system/mod.rs"),
            include_str!("../../api/windows/winapi64/advapi32/tokens/mod.rs"),
            include_str!("../../api/windows/winapi64/advapi32/tracing/mod.rs"),
        ],
    },
    DllDef {
        name: "ws2_32",
        section: "ws2_32.text",
        sources: &[include_str!("../../api/windows/winapi64/ws2_32/mod.rs")],
    },
    DllDef {
        name: "wininet",
        section: "wininet.text",
        sources: &[include_str!("../../api/windows/winapi64/wininet/mod.rs")],
    },
    DllDef {
        name: "shell32",
        section: "shell32.text",
        sources: &[
            include_str!("../../api/windows/winapi64/shell32/mod.rs"),
            include_str!("../../api/windows/winapi64/shell32/launch/mod.rs"),
            include_str!("../../api/windows/winapi64/shell32/filesystem/mod.rs"),
            include_str!("../../api/windows/winapi64/shell32/ui/mod.rs"),
            include_str!("../../api/windows/winapi64/shell32/objects/mod.rs"),
        ],
    },
    DllDef {
        name: "ole32",
        section: "ole32.text",
        sources: &[
            include_str!("../../api/windows/winapi64/ole32/mod.rs"),
            include_str!("../../api/windows/winapi64/ole32/objects/mod.rs"),
            include_str!("../../api/windows/winapi64/ole32/runtime/mod.rs"),
            include_str!("../../api/windows/winapi64/ole32/marshaling/mod.rs"),
            include_str!("../../api/windows/winapi64/ole32/storage/mod.rs"),
        ],
    },
    DllDef {
        name: "oleaut32",
        section: "oleaut32.text",
        sources: &[include_str!("../../api/windows/winapi64/oleaut32/mod.rs")],
    },
    DllDef {
        name: "crypt32",
        section: "crypt32.text",
        sources: &[include_str!("../../api/windows/winapi64/crypt32/mod.rs")],
    },
    DllDef {
        name: "kernelbase",
        section: "kernelbase.text",
        sources: &[
            include_str!("../../api/windows/winapi64/kernelbase/mod.rs"),
            include_str!("../../api/windows/winapi64/kernelbase/appcompat/mod.rs"),
            include_str!("../../api/windows/winapi64/kernelbase/console/mod.rs"),
            include_str!("../../api/windows/winapi64/kernelbase/diagnostics/mod.rs"),
            include_str!("../../api/windows/winapi64/kernelbase/filesystem/mod.rs"),
            include_str!("../../api/windows/winapi64/kernelbase/legacy_compat/mod.rs"),
            include_str!("../../api/windows/winapi64/kernelbase/library_loader/mod.rs"),
            include_str!("../../api/windows/winapi64/kernelbase/memory/mod.rs"),
            include_str!("../../api/windows/winapi64/kernelbase/network/mod.rs"),
            include_str!("../../api/windows/winapi64/kernelbase/objects/mod.rs"),
            include_str!("../../api/windows/winapi64/kernelbase/process_thread/mod.rs"),
            include_str!("../../api/windows/winapi64/kernelbase/registry_profile/mod.rs"),
            include_str!("../../api/windows/winapi64/kernelbase/security/mod.rs"),
            include_str!("../../api/windows/winapi64/kernelbase/strings_nls/mod.rs"),
            include_str!("../../api/windows/winapi64/kernelbase/sync/mod.rs"),
            include_str!("../../api/windows/winapi64/kernelbase/system/mod.rs"),
            include_str!("../../api/windows/winapi64/kernelbase/time_date/mod.rs"),
        ],
    },
    DllDef {
        name: "msvcrt",
        section: "msvcrt.text",
        sources: &[
            include_str!("../../api/windows/winapi64/msvcrt/mod.rs"),
            include_str!("../../api/windows/winapi64/msvcrt/runtime/mod.rs"),
            include_str!("../../api/windows/winapi64/msvcrt/strings/mod.rs"),
            include_str!("../../api/windows/winapi64/msvcrt/io/mod.rs"),
            include_str!("../../api/windows/winapi64/msvcrt/memory/mod.rs"),
            include_str!("../../api/windows/winapi64/msvcrt/math/mod.rs"),
        ],
    },
    DllDef {
        name: "winhttp",
        section: "winhttp.text",
        sources: &[include_str!("../../api/windows/winapi64/winhttp/mod.rs")],
    },
    DllDef {
        name: "bcrypt",
        section: "bcrypt.text",
        sources: &[include_str!("../../api/windows/winapi64/bcrypt/mod.rs")],
    },
    DllDef {
        name: "version",
        section: "version.text",
        sources: &[include_str!("../../api/windows/winapi64/version/mod.rs")],
    },
    DllDef {
        name: "shlwapi",
        section: "shlwapi.text",
        sources: &[include_str!("../../api/windows/winapi64/shlwapi/mod.rs")],
    },
    DllDef {
        name: "iphlpapi",
        section: "iphlpapi.text",
        sources: &[include_str!("../../api/windows/winapi64/iphlpapi/mod.rs")],
    },
    DllDef {
        name: "dnsapi",
        section: "dnsapi.text",
        sources: &[include_str!("../../api/windows/winapi64/dnsapi/mod.rs")],
    },
    DllDef {
        name: "urlmon",
        section: "urlmon.text",
        sources: &[include_str!("../../api/windows/winapi64/urlmon/mod.rs")],
    },
    DllDef {
        name: "uxtheme",
        section: "uxtheme.text",
        sources: &[include_str!("../../api/windows/winapi64/uxtheme/mod.rs")],
    },
];

const DLLS_32: &[DllDef] = &[
    DllDef {
        name: "kernel32",
        section: "kernel32.text",
        sources: &[
            include_str!("../../api/windows/winapi32/kernel32/mod.rs"),
            include_str!("../../api/windows/winapi32/kernel32/appcompat/mod.rs"),
            include_str!("../../api/windows/winapi32/kernel32/console/mod.rs"),
            include_str!("../../api/windows/winapi32/kernel32/diagnostics/mod.rs"),
            include_str!("../../api/windows/winapi32/kernel32/filesystem/mod.rs"),
            include_str!("../../api/windows/winapi32/kernel32/legacy_compat/mod.rs"),
            include_str!("../../api/windows/winapi32/kernel32/library_loader/mod.rs"),
            include_str!("../../api/windows/winapi32/kernel32/memory/mod.rs"),
            include_str!("../../api/windows/winapi32/kernel32/objects/mod.rs"),
            include_str!("../../api/windows/winapi32/kernel32/process_thread/mod.rs"),
            include_str!("../../api/windows/winapi32/kernel32/registry_profile/mod.rs"),
            include_str!("../../api/windows/winapi32/kernel32/security/mod.rs"),
            include_str!("../../api/windows/winapi32/kernel32/strings_nls/mod.rs"),
            include_str!("../../api/windows/winapi32/kernel32/sync/mod.rs"),
            include_str!("../../api/windows/winapi32/kernel32/system/mod.rs"),
            include_str!("../../api/windows/winapi32/kernel32/time_date/mod.rs"),
        ],
    },
    DllDef {
        name: "user32",
        section: "user32.text",
        sources: &[
            include_str!("../../api/windows/winapi32/user32/mod.rs"),
            include_str!("../../api/windows/winapi32/user32/clipboard/mod.rs"),
            include_str!("../../api/windows/winapi32/user32/dde/mod.rs"),
            include_str!("../../api/windows/winapi32/user32/dialogs/mod.rs"),
            include_str!("../../api/windows/winapi32/user32/graphics/mod.rs"),
            include_str!("../../api/windows/winapi32/user32/input/mod.rs"),
            include_str!("../../api/windows/winapi32/user32/menus/mod.rs"),
            include_str!("../../api/windows/winapi32/user32/messages/mod.rs"),
            include_str!("../../api/windows/winapi32/user32/strings_format/mod.rs"),
            include_str!("../../api/windows/winapi32/user32/system/mod.rs"),
            include_str!("../../api/windows/winapi32/user32/windowing/mod.rs"),
        ],
    },
    DllDef {
        name: "gdi32",
        section: "gdi32.text",
        sources: &[include_str!("../../api/windows/winapi32/gdi32/mod.rs")],
    },
    DllDef {
        name: "advapi32",
        section: "advapi32.text",
        sources: &[
            include_str!("../../api/windows/winapi32/advapi32/mod.rs"),
            include_str!("../../api/windows/winapi32/advapi32/accounts/mod.rs"),
            include_str!("../../api/windows/winapi32/advapi32/crypto/mod.rs"),
            include_str!("../../api/windows/winapi32/advapi32/registry_api/mod.rs"),
            include_str!("../../api/windows/winapi32/advapi32/security/mod.rs"),
            include_str!("../../api/windows/winapi32/advapi32/services/mod.rs"),
            include_str!("../../api/windows/winapi32/advapi32/system/mod.rs"),
            include_str!("../../api/windows/winapi32/advapi32/tokens/mod.rs"),
            include_str!("../../api/windows/winapi32/advapi32/tracing/mod.rs"),
        ],
    },
    DllDef {
        name: "ws2_32",
        section: "ws2_32.text",
        sources: &[include_str!("../../api/windows/winapi32/ws2_32/mod.rs")],
    },
    DllDef {
        name: "wininet",
        section: "wininet.text",
        sources: &[include_str!("../../api/windows/winapi32/wininet/mod.rs")],
    },
    DllDef {
        name: "shell32",
        section: "shell32.text",
        sources: &[
            include_str!("../../api/windows/winapi32/shell32/mod.rs"),
            include_str!("../../api/windows/winapi32/shell32/launch/mod.rs"),
            include_str!("../../api/windows/winapi32/shell32/ui/mod.rs"),
            include_str!("../../api/windows/winapi32/shell32/objects/mod.rs"),
            include_str!("../../api/windows/winapi32/shell32/filesystem/mod.rs"),
        ],
    },
    DllDef {
        name: "ole32",
        section: "ole32.text",
        sources: &[
            include_str!("../../api/windows/winapi32/ole32/mod.rs"),
            include_str!("../../api/windows/winapi32/ole32/objects/mod.rs"),
            include_str!("../../api/windows/winapi32/ole32/runtime/mod.rs"),
            include_str!("../../api/windows/winapi32/ole32/marshaling/mod.rs"),
            include_str!("../../api/windows/winapi32/ole32/storage/mod.rs"),
        ],
    },
    DllDef {
        name: "oleaut32",
        section: "oleaut32.text",
        sources: &[include_str!("../../api/windows/winapi32/oleaut32/mod.rs")],
    },
    DllDef {
        name: "crypt32",
        section: "crypt32.text",
        sources: &[include_str!("../../api/windows/winapi32/crypt32/mod.rs")],
    },
    DllDef {
        name: "kernelbase",
        section: "kernelbase.text",
        sources: &[include_str!("../../api/windows/winapi32/kernelbase/mod.rs")],
    },
    DllDef {
        name: "msvcrt",
        section: "msvcrt.text",
        sources: &[
            include_str!("../../api/windows/winapi32/msvcrt/mod.rs"),
            include_str!("../../api/windows/winapi32/msvcrt/runtime/mod.rs"),
            include_str!("../../api/windows/winapi32/msvcrt/strings/mod.rs"),
            include_str!("../../api/windows/winapi32/msvcrt/io/mod.rs"),
            include_str!("../../api/windows/winapi32/msvcrt/memory/mod.rs"),
            include_str!("../../api/windows/winapi32/msvcrt/math/mod.rs"),
        ],
    },
    DllDef {
        name: "winhttp",
        section: "winhttp.text",
        sources: &[include_str!("../../api/windows/winapi32/winhttp/mod.rs")],
    },
    DllDef {
        name: "bcrypt",
        section: "bcrypt.text",
        sources: &[include_str!("../../api/windows/winapi32/bcrypt/mod.rs")],
    },
    DllDef {
        name: "shlwapi",
        section: "shlwapi.text",
        sources: &[include_str!("../../api/windows/winapi32/shlwapi/mod.rs")],
    },
    DllDef {
        name: "dnsapi",
        section: "dnsapi.text",
        sources: &[include_str!("../../api/windows/winapi32/dnsapi/mod.rs")],
    },
    DllDef {
        name: "urlmon",
        section: "urlmon.text",
        sources: &[include_str!("../../api/windows/winapi32/urlmon/mod.rs")],
    },
];

fn extract_api_names(source: &str) -> Vec<String> {
    source
        .lines()
        .filter_map(|line| line.trim().split_once("=>"))
        .flat_map(|(patterns, _)| patterns.split('|'))
        .filter_map(|pattern| pattern.trim().strip_prefix('"')?.strip_suffix('"'))
        .map(str::to_string)
        .collect()
}

fn register_dll(emu: &mut Emu, dll: &str, api: &str, base: u64) {
    emu.maps
        .create_map(
            &format!("{dll}.text"),
            base,
            0x1000,
            Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    let exports = ExportIndexData {
        export_base: 1,
        number_of_functions: 1,
        ordinal_targets: vec![Some(ExportTarget::Direct { rva: 0x100 })],
        named_exports: vec![NamedExport {
            name: api.into(),
            ordinal_index: 0,
        }],
    };
    register_export_module(emu, &format!("{dll}.dll"), base, &exports);
}

fn make_emu64_zeroed(dll: &str, api: &str) -> Emu {
    let mut emu = emu64();
    emu.cfg.console_enabled = false;
    emu.running_script = true;
    emu.init_stack64();
    register_dll(&mut emu, dll, api, MODULE64);

    // All arg registers zero — safe as both null pointers and zero sizes.
    emu.regs_mut().rcx = 0;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().r8 = 0;
    emu.regs_mut().r9 = 0;
    let rsp = emu.regs().rsp;
    for i in 0..16 {
        emu.maps.write_qword(rsp + 0x20 + i * 8, 0);
    }
    emu
}

const ARENA: u64 = 0x600000;

fn make_emu64_with_arena(dll: &str, api: &str) -> (Emu, u64) {
    let mut emu = emu64();
    emu.cfg.console_enabled = false;
    emu.running_script = true;
    emu.init_stack64();
    register_dll(&mut emu, dll, api, MODULE64);

    emu.maps
        .create_map("arena", ARENA, 0x10000, Permission::READ_WRITE)
        .unwrap();

    // rcx points into arena (valid pointer, small enough if misread as size).
    // rdx points into arena (the sentinel target).
    // r8/r9 zero (safe sizes).
    emu.regs_mut().rcx = ARENA + 0x100;
    emu.regs_mut().rdx = ARENA + 0x2000;
    emu.regs_mut().r8 = 0;
    emu.regs_mut().r9 = 0;
    let rsp = emu.regs().rsp;
    for i in 0..16 {
        emu.maps.write_qword(rsp + 0x20 + i * 8, 0);
    }
    (emu, ARENA)
}

fn make_emu32_zeroed(dll: &str, api: &str) -> Emu {
    let mut emu = emu32();
    emu.cfg.console_enabled = false;
    emu.running_script = true;
    emu.init_stack32();
    register_dll(&mut emu, dll, api, MODULE32);

    for _ in 0..20 {
        emu.stack_push32(0);
    }
    emu
}

fn set_sentinels_64(emu: &mut Emu) {
    emu.regs_mut().rbx = SENTINEL;
    emu.regs_mut().rbp = SENTINEL;
    emu.regs_mut().rsi = SENTINEL;
    emu.regs_mut().rdi = SENTINEL;
    emu.regs_mut().r12 = SENTINEL;
    emu.regs_mut().r13 = SENTINEL;
    emu.regs_mut().r14 = SENTINEL;
    emu.regs_mut().r15 = SENTINEL;
}

fn set_sentinels_32(emu: &mut Emu) {
    emu.regs_mut().rbx = SENTINEL & 0xFFFFFFFF;
    emu.regs_mut().rbp = SENTINEL & 0xFFFFFFFF;
    emu.regs_mut().rsi = SENTINEL & 0xFFFFFFFF;
    emu.regs_mut().rdi = SENTINEL & 0xFFFFFFFF;
}

#[test]
fn winapi64_handlers_preserve_rsp() {
    setup();
    let mut failures = Vec::new();
    let mut tested = 0;

    for dll in DLLS_64 {
        let apis = dll.api_names();
        for api in &apis {
            let mut emu = make_emu64_zeroed(dll.name, api);
            let rsp_before = emu.regs().rsp;

            if catch_unwind(AssertUnwindSafe(|| {
                winapi64::gateway(MODULE64 + 0x100, dll.section, &mut emu);
            }))
            .is_err()
            {
                continue;
            }

            let rsp_after = emu.regs().rsp;
            if rsp_after != rsp_before {
                failures.push(format!(
                    "{}!{}: rsp changed 0x{:x} -> 0x{:x} (delta={})",
                    dll.name,
                    api,
                    rsp_before,
                    rsp_after,
                    rsp_after as i64 - rsp_before as i64
                ));
            }
            tested += 1;
        }
    }

    assert!(tested > 100, "expected >100 APIs tested, got {tested}");
    assert!(
        failures.is_empty(),
        "{} of {tested} 64-bit APIs changed rsp:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn winapi64_handlers_preserve_nonvolatile_regs() {
    setup();
    let mut failures = Vec::new();
    let mut tested = 0;

    for dll in DLLS_64 {
        let apis = dll.api_names();
        for api in &apis {
            let mut emu = make_emu64_zeroed(dll.name, api);
            set_sentinels_64(&mut emu);

            if catch_unwind(AssertUnwindSafe(|| {
                winapi64::gateway(MODULE64 + 0x100, dll.section, &mut emu);
            }))
            .is_err()
            {
                continue;
            }

            let r = emu.regs();
            for (name, val) in [
                ("rbx", r.rbx),
                ("rbp", r.rbp),
                ("rsi", r.rsi),
                ("rdi", r.rdi),
                ("r12", r.r12),
                ("r13", r.r13),
                ("r14", r.r14),
                ("r15", r.r15),
            ] {
                if val != SENTINEL {
                    failures.push(format!(
                        "{}!{}: {name} = 0x{val:x} (expected 0x{SENTINEL:x})",
                        dll.name, api
                    ));
                }
            }
            tested += 1;
        }
    }

    assert!(tested > 100, "expected >100 APIs tested, got {tested}");
    assert!(
        failures.is_empty(),
        "{} of {tested} 64-bit APIs corrupted non-volatile regs:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
#[ignore]
fn winapi32_handlers_preserve_nonvolatile_regs() {
    setup();
    let mut failures = Vec::new();
    let mut tested = 0;
    let mask = 0xFFFFFFFF_u64;

    for dll in DLLS_32 {
        let apis = dll.api_names();
        for api in &apis {
            let mut emu = make_emu32_zeroed(dll.name, api);
            set_sentinels_32(&mut emu);

            if catch_unwind(AssertUnwindSafe(|| {
                winapi32::gateway((MODULE32 + 0x100) as u32, dll.section, &mut emu);
            }))
            .is_err()
            {
                continue;
            }

            let r = emu.regs();
            for (name, val) in [
                ("ebx", r.rbx & mask),
                ("ebp", r.rbp & mask),
                ("esi", r.rsi & mask),
                ("edi", r.rdi & mask),
            ] {
                let expected = SENTINEL & mask;
                if val != expected {
                    failures.push(format!(
                        "{}!{}: {name} = 0x{val:x} (expected 0x{expected:x})",
                        dll.name, api
                    ));
                }
            }
            tested += 1;
        }
    }

    assert!(tested > 100, "expected >100 APIs tested, got {tested}");
    assert!(
        failures.is_empty(),
        "{} of {tested} 32-bit APIs corrupted non-volatile regs:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

/// Spot-check that APIs known NOT to use rdx as an output pointer don't
/// write to it. This catches the mechanical `write_qword(rdx, 0)` pattern
/// that the 58433ebf regression introduced across ~1,400 handlers.
#[test]
fn winapi64_non_output_apis_dont_write_rdx() {
    setup();
    let mut failures = Vec::new();

    // Representative APIs where rdx is NOT an output pointer.
    let cases: &[(&str, &str, &str)] = &[
        ("kernel32", "kernel32.text", "GetLocaleInfoW"),
        ("kernel32", "kernel32.text", "SetLastError"),
        ("kernel32", "kernel32.text", "Sleep"),
        ("kernel32", "kernel32.text", "GetTickCount"),
        ("kernel32", "kernel32.text", "GetCurrentProcessId"),
        ("kernel32", "kernel32.text", "GetCurrentThreadId"),
        ("kernel32", "kernel32.text", "ExitProcess"),
        ("kernel32", "kernel32.text", "FreeLibrary"),
        ("kernel32", "kernel32.text", "CloseHandle"),
        ("user32", "user32.text", "ShowWindow"),
        ("user32", "user32.text", "SetWindowTextA"),
        ("user32", "user32.text", "DestroyWindow"),
        ("user32", "user32.text", "PostMessageA"),
        ("user32", "user32.text", "MessageBoxA"),
        ("gdi32", "gdi32.text", "RestoreDC"),
        ("gdi32", "gdi32.text", "SelectObject"),
        ("gdi32", "gdi32.text", "DeleteObject"),
        ("advapi32", "advapi32.text", "RegCloseKey"),
        ("ws2_32", "ws2_32.text", "closesocket"),
        ("ws2_32", "ws2_32.text", "htons"),
    ];

    for &(dll, section, api) in cases {
        let (mut emu, _arena) = make_emu64_with_arena(dll, api);

        let rdx_addr = emu.regs().rdx;
        emu.maps.write_qword(rdx_addr, SENTINEL);

        if catch_unwind(AssertUnwindSafe(|| {
            winapi64::gateway(MODULE64 + 0x100, section, &mut emu);
        }))
        .is_err()
        {
            continue;
        }

        if let Some(val) = emu.maps.read_qword(rdx_addr)
            && val != SENTINEL
        {
            failures.push(format!(
                "{dll}!{api}: wrote 0x{val:x} to rdx (sentinel was 0x{SENTINEL:x})"
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "APIs that should not write to rdx did:\n{}",
        failures.join("\n")
    );
}

#[test]
fn large_gateways_fit_a_one_mebibyte_host_stack() {
    const HOST_STACK_SIZE: usize = 1024 * 1024;
    setup();
    for (is64, dll) in iproduct!(
        [false, true],
        [
            "kernel32",
            "kernelbase",
            "advapi32",
            "user32",
            "shell32",
            "ole32"
        ]
    ) {
        thread::Builder::new()
            .stack_size(HOST_STACK_SIZE)
            .spawn(move || {
                let mut emu = if is64 {
                    make_emu64_zeroed(dll, "MissingApiForStackCheck")
                } else {
                    make_emu32_zeroed(dll, "MissingApiForStackCheck")
                };
                emu.cfg.skip_unimplemented = false;
                emu.is_running.store(1, Ordering::Relaxed);
                let section = format!("{dll}.text");
                if is64 {
                    winapi64::gateway(MODULE64 + 0x100, &section, &mut emu);
                } else {
                    winapi32::gateway((MODULE32 + 0x100) as u32, &section, &mut emu);
                }
                assert_eq!(emu.is_running.load(Ordering::Relaxed), 0);
            })
            .unwrap()
            .join()
            .unwrap();
    }
}

#[test]
fn kernelbase_preserves_the_unimplemented_api_result() {
    let mut emu = make_emu64_zeroed("kernelbase", "MissingKernelbaseApi");
    emu.cfg.skip_unimplemented = false;
    assert_eq!(
        winapi64::kernelbase::gateway(MODULE64 + 0x100, &mut emu),
        "missingkernelbaseapi"
    );
}

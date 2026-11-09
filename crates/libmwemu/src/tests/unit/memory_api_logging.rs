use crate::emu::Emu;
use crate::tests::helpers::{call_winapi32, call_winapi64, setup};
use crate::winapi::common::{
    crt,
    memory::{self, Call},
};
use crate::{emu32, emu64};
use std::{env, process::Command};

const CHILD: &str = "MWEMU_MEMORY_API_LOGGING_CHILD";

fn invoke(emu: &mut Emu, handler: fn(&mut Emu), args: &[u64]) {
    let stack = emu.regs().rsp;
    if emu.cfg.arch.is_64bits() {
        call_winapi64(emu, handler, args);
    } else {
        let args: Vec<_> = args.iter().map(|&n| n as u32).collect();
        call_winapi32(emu, handler, &args);
    }
    assert_eq!(emu.regs().rsp, stack);
}

fn exercise_calls() {
    setup();
    for is64 in [false, true] {
        let mut emu = if is64 { emu64() } else { emu32() };
        emu.cfg.console_enabled = false;
        emu.cfg.nocolors = true;
        emu.cfg.verbose = 1;
        if is64 {
            emu.init_stack64();
        } else {
            emu.init_stack32();
        }
        invoke(
            &mut emu,
            |emu| {
                assert!(memory::dispatch("VirtualAllocEx", emu));
            },
            &[0x1234, 0, 0x2000, 0x3000, 4],
        );
        assert_eq!(emu.regs().rax, 0);
        invoke(
            &mut emu,
            |emu| {
                assert!(memory::dispatch_nt(
                    "ZwProtectVirtualMemory",
                    emu,
                    Call::Api
                ));
            },
            &[0x1234, 0, 0, 0x20, 0x4242],
        );
        assert_eq!(emu.regs().rax, 0xc0000008);
        let stack = emu.regs().rsp;
        emu.regs_mut().rax = 0xdeadbeef;
        if is64 {
            emu.regs_mut().rcx = 0;
        } else {
            emu.stack_push32(0);
        }
        assert!(crt::dispatch("free", &mut emu));
        assert_eq!(emu.regs().rax, 0xdeadbeef);
        if !is64 {
            assert_eq!(emu.stack_pop32(false), Some(0));
        }
        assert_eq!(emu.regs().rsp, stack);
        emu.cfg.verbose = 0;
        invoke(
            &mut emu,
            |emu| {
                assert!(memory::dispatch("VirtualFree", emu));
            },
            &[0, 0, 0x8000],
        );
    }
    let mut emu = emu64();
    emu.cfg.console_enabled = false;
    emu.cfg.nocolors = true;
    emu.cfg.verbose = 0;
    emu.init_stack64();
    emu.regs_mut().rax = 0x123;
    emu.regs_mut().rcx = 0x1234;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().r8 = 0;
    emu.regs_mut().r9 = 0x3000;
    let stack = emu.regs().rsp;
    // Syscall stack arguments start at +0x28, unlike API arguments at +0x20.
    for (offset, value) in [(0x20, 0xbad), (0x28, 4), (0x30, 0xaabbccdd), (0x38, 0xee)] {
        assert!(emu.maps.write_qword(stack + offset, value));
    }
    assert!(memory::dispatch_nt(
        "NtAllocateVirtualMemoryEx",
        &mut emu,
        Call::Syscall
    ));
    assert_eq!(emu.regs().rax, 0xc0000008);
    assert_eq!(emu.regs().rsp, stack);
}

#[test]
fn memory_api_trace_preserves_names_arguments_and_verbosity() {
    if env::var_os(CHILD).is_some() {
        exercise_calls();
        return;
    }
    // A separate process isolates the global logger from the rest of the suite.
    let output = Command::new(env::current_exe().unwrap())
        .args(["--exact", "tests::unit::memory_api_logging::memory_api_trace_preserves_names_arguments_and_verbosity", "--nocapture"])
        .env(CHILD, "1").env("RUST_LOG", "trace").output().unwrap();
    let logs = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        logs
    );
    for expected in [
        "VirtualAllocEx(process: 0x1234, address: 0x0, size: 0x2000, flags: 0x3000, protection: 0x4) = 0x0",
        "ZwProtectVirtualMemory(process: 0x1234, base_ptr: 0x0, size_ptr: 0x0, protection: 0x20, old_protection: 0x4242) = 0xc0000008",
        "free(address: 0x0)",
    ] {
        assert_eq!(logs.matches(expected).count(), 2, "{logs}");
    }
    assert!(logs.contains("syscall 0x123: NtAllocateVirtualMemoryEx(process: 0x1234, base_ptr: 0x0, size_ptr: 0x0, flags: 0x3000, protection: 0x4, parameters: 0xaabbccdd, parameter_count: 0xee) = 0xc0000008"), "{logs}");
    assert!(!logs.contains("VirtualFree("), "{logs}");
    assert!(!logs.contains("free(address: 0x0) ="), "{logs}");
    assert!(!logs.contains('\u{1b}'), "{logs}");
}

//! Setting an emulator up as a macOS process: the thread state, the stack
//! and the arguments `_main` expects.

use crate::emu::{Emu, InstructionState};
use crate::threading::context::ThreadContext;

/// Initialising an emulator as a macOS process.
pub(crate) trait MacosInit {
    /// Set up macOS AArch64 calling convention for _main(argc, argv, envp, apple).
    /// dyld calls _main with arguments in registers, not on the stack.
    fn write_macos_stack_layout(&mut self);

    /// Initialize macOS aarch64 simulation for Mach-O loading.
    fn init_macos_aarch64(&mut self);

    /// Initialize macOS x86_64 simulation for Mach-O loading.
    fn init_macos64(&mut self);
}

impl MacosInit for Emu {
    /// Set up macOS AArch64 calling convention for _main(argc, argv, envp, apple).
    /// dyld calls _main with arguments in registers, not on the stack.
    fn write_macos_stack_layout(&mut self) {
        let sp = self.regs_aarch64().sp;

        // Build the argument list: argv[0] = filename, rest from cfg.arguments
        let mut args: Vec<String> = vec![self.cfg.filename.clone()];
        if !self.cfg.arguments.is_empty() {
            for a in self.cfg.arguments.split_whitespace() {
                args.push(a.to_string());
            }
        }
        let argc = args.len() as u64;

        // Write argument strings into the stack area
        let strings_base = sp + 0x200;
        let mut string_addrs: Vec<u64> = Vec::new();
        let mut string_off: u64 = 0;
        for arg in &args {
            let addr = strings_base + string_off;
            let bytes = arg.as_bytes();
            for (i, &b) in bytes.iter().enumerate() {
                self.maps.write_byte(addr + i as u64, b);
            }
            self.maps.write_byte(addr + bytes.len() as u64, 0);
            string_addrs.push(addr);
            string_off += (bytes.len() as u64) + 1;
        }

        // Write argv pointer array on the stack
        let argv_base = sp + 0x100;
        for (i, &addr) in string_addrs.iter().enumerate() {
            self.maps.write_qword(argv_base + (i as u64) * 8, addr);
        }
        // argv terminator
        self.maps
            .write_qword(argv_base + (string_addrs.len() as u64) * 8, 0);

        // envp = empty array (just NULL terminator)
        let envp_base = argv_base + ((string_addrs.len() as u64) + 1) * 8;
        self.maps.write_qword(envp_base, 0);

        // apple[] = empty array (just NULL terminator)
        let apple_base = envp_base + 8;
        self.maps.write_qword(apple_base, 0);

        // Set registers: _main(argc, argv, envp, apple)
        let regs = self.regs_aarch64_mut();
        regs.x[0] = argc;
        regs.x[1] = argv_base;
        regs.x[2] = envp_base;
        regs.x[3] = apple_base;
    }

    /// Initialize macOS aarch64 simulation for Mach-O loading.
    fn init_macos_aarch64(&mut self) {
        self.os = crate::arch::OperatingSystem::MacOS;

        // Ensure thread context is aarch64
        if matches!(
            self.threads[self.current_thread_id].arch,
            crate::threading::context::ArchThreadState::X86 { .. }
        ) {
            let id = self.threads[self.current_thread_id].id;
            self.threads[self.current_thread_id] = ThreadContext::new(id, self.cfg.arch);
        }

        self.init_stack_aarch64();
    }

    /// Initialize macOS x86_64 simulation for Mach-O loading.
    fn init_macos64(&mut self) {
        self.os = crate::arch::OperatingSystem::MacOS;
        self.maps.is_64bits = true;

        // Mach-O front-door loading can switch the emulator from an earlier
        // AArch64 session, so restore the x86 thread/register state explicitly.
        if matches!(
            self.threads[self.current_thread_id].arch,
            crate::threading::context::ArchThreadState::AArch64 { .. }
        ) {
            let id = self.threads[self.current_thread_id].id;
            self.threads[self.current_thread_id] = ThreadContext::new(id, self.cfg.arch);
        }

        self.instruction_state = InstructionState::default();
        self.flags_mut().clear();
        self.flags_mut().f_if = true;
        self.init_stack64();
    }
}

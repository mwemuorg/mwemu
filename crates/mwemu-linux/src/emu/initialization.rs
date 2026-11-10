//! Setting an emulator up as a Linux process: the stack, the heap and the
//! initial stack layout `_start` expects.

use crate::emu::Emu;
use crate::maps::heap_allocation::O1Heap;
use crate::maps::mem64::Permission;
use crate::threading::context::ThreadContext;

/// Initialising an emulator as a Linux process.
pub(crate) trait LinuxInit {
    /// Initialize linux aarch64 simulation for ELF loading.
    fn init_linux64_aarch64(&mut self);

    /// Write the Linux initial stack layout that _start expects.
    ///
    /// The kernel places this on the stack before jumping to the ELF entry point:
    ///   [SP+0]  argc
    ///   [SP+8]  argv[0]  (pointer to program name)
    ///   [SP+16] NULL     (argv terminator)
    ///   [SP+24] NULL     (envp terminator)
    ///   auxv[]: AT_PHDR, AT_PHENT, AT_PHNUM, AT_PAGESZ, AT_ENTRY, AT_NULL
    fn write_linux_stack_layout(&mut self, entry: u64, phdr_addr: u64, phentsize: u16, phnum: u16);

    /// Initialize linux x86_64 simulation, it's called from load_code() if the sample is an ELF.
    fn init_linux64(&mut self, dyn_link: bool);
}

impl LinuxInit for Emu {
    /// Initialize linux aarch64 simulation for ELF loading.
    fn init_linux64_aarch64(&mut self) {
        // Ensure thread context is aarch64
        if matches!(
            self.threads[self.current_thread_id].arch,
            crate::threading::context::ArchThreadState::X86 { .. }
        ) {
            let id = self.threads[self.current_thread_id].id;
            self.threads[self.current_thread_id] = ThreadContext::new(id, self.cfg.arch);
        }

        self.ensure_arch_state_aarch64();
        self.init_stack_aarch64();
    }

    /// Write the Linux initial stack layout that _start expects.
    ///
    /// The kernel places this on the stack before jumping to the ELF entry point:
    ///   [SP+0]  argc
    ///   [SP+8]  argv[0]  (pointer to program name)
    ///   [SP+16] NULL     (argv terminator)
    ///   [SP+24] NULL     (envp terminator)
    ///   auxv[]: AT_PHDR, AT_PHENT, AT_PHNUM, AT_PAGESZ, AT_ENTRY, AT_NULL
    fn write_linux_stack_layout(&mut self, entry: u64, phdr_addr: u64, phentsize: u16, phnum: u16) {
        let sp = if self.cfg.arch.is_aarch64() {
            self.regs_aarch64().sp
        } else {
            self.regs().rsp
        };

        // Write a program name string above the stack layout area
        let prog_name_addr = sp + 0x100;
        let prog_name = b"prog\0";
        for (i, &b) in prog_name.iter().enumerate() {
            self.maps.write_byte(prog_name_addr + i as u64, b);
        }

        let mut off = sp;

        // argc = 1
        self.maps.write_qword(off, 1);
        off += 8;

        // argv[0] = pointer to program name
        self.maps.write_qword(off, prog_name_addr);
        off += 8;

        // argv terminator
        self.maps.write_qword(off, 0);
        off += 8;

        // envp terminator
        self.maps.write_qword(off, 0);
        off += 8;

        // Auxiliary vector entries (each is 16 bytes: type + value)
        const AT_PHDR: u64 = 3;
        const AT_PHENT: u64 = 4;
        const AT_PHNUM: u64 = 5;
        const AT_PAGESZ: u64 = 6;
        const AT_ENTRY: u64 = 9;
        const AT_NULL: u64 = 0;

        let auxv: &[(u64, u64)] = &[
            (AT_PAGESZ, 4096),
            (AT_PHDR, phdr_addr),
            (AT_PHENT, phentsize as u64),
            (AT_PHNUM, phnum as u64),
            (AT_ENTRY, entry),
            (AT_NULL, 0),
        ];

        for &(atype, aval) in auxv {
            self.maps.write_qword(off, atype);
            off += 8;
            self.maps.write_qword(off, aval);
            off += 8;
        }
    }

    /// Initialize linux x86_64 simulation, it's called from load_code() if the sample is an ELF.
    fn init_linux64(&mut self, dyn_link: bool) {
        //self.regs_mut().clear::<64>();
        self.flags_mut().clear();
        self.flags_mut().f_if = true;

        // Avoid `set_current_dir` here: the process-wide CWD races with any
        // sibling test running in parallel (cargo test uses many threads) —
        // most visibly causing the Win64 auto-detect in `init_win32_mem64`
        // to miss `maps/windows/x86_64`. Build absolute paths instead.
        if dyn_link {
            //self.regs_mut().rsp = 0x7fffffffe2b0;
            self.regs_mut().rsp = 0x7fffffffe790;
            if let Err(e) = self.maps.create_map(
                "linux_dynamic_stack",
                0x7ffffffde000,
                0x100000,
                Permission::READ_WRITE,
            ) {
                log::error!("cannot create linux_dynamic_stack map: {}", e);
                return;
            }
            if let Err(e) =
                self.maps
                    .create_map("dso_dyn", 0x7ffff7ffd000, 0x1000, Permission::READ_WRITE)
            {
                log::error!("cannot create dso_dyn map: {}", e);
            }
            if let Err(e) = self.maps.create_map(
                "linker",
                0x7ffff7ffd000 - 0x1000 - 0x10000,
                0x10000,
                Permission::READ_WRITE,
            ) {
                log::error!("cannot create linker map: {}", e);
            }
        } else {
            self.regs_mut().rsp = 0x7fffffffe270;
            if let Err(e) = self.maps.create_map(
                "linux_static_stack",
                0x7ffffffde000,
                0x100000,
                Permission::READ_WRITE,
            ) {
                log::error!("cannot create linux_static_stack map: {}", e);
                return;
            }
            if let Err(e) =
                self.maps
                    .create_map("dso", 0x7ffff7ffd000, 0x100000, Permission::READ_WRITE)
            {
                log::error!("cannot create dso map: {}", e);
            }
        }
        match self
            .maps
            .create_map("tls", 0x7ffff8fff000, 0xfff, Permission::READ_WRITE)
        {
            Ok(tls) => {
                let tls_path = self.cfg.get_maps_folder("tls.bin");
                tls.load(&tls_path);
            }
            Err(e) => {
                log::error!("cannot create tls map: {}", e);
            }
        }

        if dyn_link {
            //heap.set_base(0x555555579000);
        } else {
            let heap_sz = 0x885900 - 0x4b5000;
            self.heap_addr = match self.maps.alloc(heap_sz) {
                Some(addr) => addr,
                None => {
                    log::error!("cannot allocate heap for linux64");
                    return;
                }
            };
            match self
                .maps
                .create_map(".heap", self.heap_addr, heap_sz, Permission::READ_WRITE)
            {
                Ok(heap) => {
                    heap.load("heap.bin");
                }
                Err(e) => {
                    log::error!("cannot create heap map: {}", e);
                    return;
                }
            }

            if self.heap_arenas.is_empty() {
                match O1Heap::new(self.heap_addr, heap_sz as u32) {
                    Ok(heap) => self.heap_arenas.push(Box::new(heap)),
                    Err(e) => log::error!("cannot init heap management: {}", e),
                }
            }
        }

        self.regs_mut().rbp = 0;

        // Linux x86-64 TCB (Thread Control Block)
        let tcb_addr: u64 = 0x7ffff9000000;
        let tcb_size: u64 = 0x1000;
        if let Err(e) =
            self.maps
                .create_map("linux_tcb", tcb_addr, tcb_size, Permission::READ_WRITE)
        {
            log::error!("cannot create linux_tcb map: {}", e);
            return;
        }
        self.maps.write_qword(tcb_addr, tcb_addr); // +0x00: self pointer
        self.maps.write_qword(tcb_addr + 0x10, tcb_addr); // +0x10: self pointer
        self.maps.write_qword(tcb_addr + 0x28, 0xdeadbeefcafebabe); // +0x28: stack canary
        self.regs_mut().fs = tcb_addr;
    }
}

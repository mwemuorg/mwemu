use env_logger::Env;
use std::io::Write as _;

use pyo3::exceptions::PyException;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use pyo3_stub_gen::define_stub_info_gatherer;
use pyo3_stub_gen::derive::gen_stub_pyclass;
use pyo3_stub_gen::derive::gen_stub_pymethods;
use pyo3_stub_gen::derive::gen_stub_pyfunction;

use libmwemu::console::Console;
use libmwemu::emu32;
use libmwemu::emu64;
use libmwemu::emu_aarch64;
use libmwemu::maps::mem64::Permission;
use pyo3::class::basic::CompareOp;


#[gen_stub_pyclass]
#[pyclass(name = "Permission", module = "pymwemu._pymwemu")]
#[derive(Clone, Copy)]
pub struct PyPermission(pub Permission);

#[gen_stub_pymethods]
#[pymethods]
impl PyPermission {
    #[classattr]
    const NONE: PyPermission = PyPermission(Permission::NONE);

    #[classattr]
    const READ: PyPermission = PyPermission(Permission::READ);

    #[classattr]
    const WRITE: PyPermission = PyPermission(Permission::WRITE);
    
    #[classattr]
    const EXECUTE: PyPermission = PyPermission(Permission::EXECUTE);

    #[classattr]
    const READ_WRITE: PyPermission = PyPermission(Permission::READ_WRITE);

    #[classattr]
    const READ_EXECUTE: PyPermission = PyPermission(Permission::READ_EXECUTE);

    #[classattr]
    const WRITE_EXECUTE: PyPermission = PyPermission(Permission::WRITE_EXECUTE);

    #[classattr]
    const READ_WRITE_EXECUTE: PyPermission = PyPermission(Permission::READ_WRITE_EXECUTE);

    #[new]
    #[pyo3(signature = (read=false, write=false, execute=false))]
    fn new(read: bool, write: bool, execute: bool) -> Self {
        PyPermission(Permission::from_flags(read, write, execute))
    }

    #[staticmethod]
    fn from_bits(bits: u8) -> Self {
        PyPermission(Permission::from_bits(bits))
    }

    #[getter]
    fn bits(&self) -> u8 {
        self.0.bits()
    }

    #[getter]
    fn can_read(&self) -> bool {
        self.0.can_read()
    }

    #[getter]
    fn can_write(&self) -> bool {
        self.0.can_write()
    }

    #[getter]
    fn is_accessible(&self) -> bool {
        self.0.is_accessible()
    }

    fn __or__(&self, rhs: Self) -> Self {
        PyPermission(self.0 | rhs.0)
    }

    fn __and__(&self, rhs: Self) -> Self {
        PyPermission(self.0 & rhs.0)
    }

    fn __xor__(&self, rhs: Self) -> Self {
        PyPermission(self.0 ^ rhs.0)
    }

    fn __invert__(&self) -> Self {
        PyPermission(!self.0)
    }

    fn __richcmp__(&self, other: Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0.bits() == other.0.bits(),
            CompareOp::Ne => self.0.bits() != other.0.bits(),
            _ => false,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Permission(bits={:#05b}, R={}, W={}, X={})",
            self.0.bits(),
            self.0.can_read(),
            self.0.can_write(),
            self.0.can_execute()
        )
    }   

}

#[gen_stub_pyclass]
#[pyclass(unsendable, module="pymwemu._pymwemu")]
pub struct Emu {
    emu: libmwemu::emu::Emu,
}

#[gen_stub_pymethods]
#[pymethods]
#[allow(deprecated)]
impl Emu {
    /// get pymwemu version.
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// get last emulated mnemonic with name and parameters.
    fn get_prev_mnemonic(&mut self) -> PyResult<String> {
        match self.emu.last_decoded {
            Some(decoded) => Ok(self.emu.format_instruction(&decoded)),
            None => Err(PyValueError::new_err("no instruction decoded yet")),
        }
    }

    /// reset the instruction counter to zero.
    fn reset_pos(&mut self) {
        self.emu.pos = 0;
    }

    /// check if the emulator is in 64bits mode.
    fn is_64bits(&self) -> PyResult<bool> {
        Ok(self.emu.cfg.is_x64())
    }

    /// check if the emulator is in 32bits mode.
    fn is_32bits(&self) -> PyResult<bool> {
        Ok(!self.emu.cfg.is_x64())
    }

    /// change base address on ldr entry of a module
    fn update_ldr_entry_base(&mut self, libname: &str, base: u64) {
        self.emu.update_ldr_entry_base(libname, base);
    }

    /// Set 64bits mode, it's necessary to load the 64bits maps with load_maps() method.
    /// Or better can use: emu = pymwemu.init64()
    fn set_64bits(&mut self) {
        self.emu.cfg.arch = libmwemu::arch::Arch::X86_64;
    }

    /// Set 32bits mode, it's necessary to load the 32bits maps with load_maps() method.
    /// Or better can use: emu = pymwemu.init32()
    fn set_32bits(&mut self) {
        self.emu.cfg.arch = libmwemu::arch::Arch::X86;
    }

    /// Set aarch64 (ARM64) mode.
    /// Or better can use: emu = pymwemu.init_aarch64()
    fn set_aarch64(&mut self) {
        self.emu.cfg.arch = libmwemu::arch::Arch::Aarch64;
    }

    /// disable the colored mode for instructions, api calls and other logs.
    fn disable_colors(&mut self) {
        self.emu.cfg.nocolors = true;
    }

    /// enable the colored mode.
    fn enable_colors(&mut self) {
        self.emu.cfg.nocolors = false;
    }

    /// trace all memory reads and writes.
    fn enable_trace_mem(&mut self) {
        self.emu.cfg.trace_mem = true;
    }

    /// disable the memory tracer.
    fn disable_trace_mem(&mut self) {
        self.emu.cfg.trace_mem = false;
    }

    /// trace all the registers printing them in every step.
    fn enable_trace_regs(&mut self) {
        self.emu.cfg.trace_regs = true;
    }

    /// disable the register tracer.
    fn disable_trace_regs(&mut self) {
        self.emu.cfg.trace_regs = false;
    }

    /// trace a specific list of registers, provide  array of strings with register names in lower case.
    fn enable_trace_reg(&mut self, regs: Vec<String>) {
        self.emu.cfg.trace_reg = true;
        self.emu.cfg.reg_names = regs;
    }

    /// disable the multi-register tracer.
    fn disable_trace_reg(&mut self) {
        self.emu.cfg.trace_reg = false;
        self.emu.cfg.reg_names.clear();
    }

    /// inspect sequence
    fn inspect_seq(&mut self, s: &str) {
        self.emu.cfg.inspect = true;
        self.emu.cfg.inspect_seq = s.to_string();
    }

    /// address to api name
    fn api_addr_to_name(&mut self, addr: u64) -> String {
        self.emu.api_addr_to_name(addr)
    }

    /// api name to address
    fn api_name_to_addr(&mut self, name: &str) -> u64 {
        self.emu.api_name_to_addr(name)
    }

    /// set the verbosity between 0 and 3.
    ///     0: only show api calls.
    ///     1: show api calls and some logs.
    ///     2: show also instructions (slower).
    ///     3: show every iteration of rep preffix.
    fn set_verbose(&mut self, verbose: u32) {
        self.emu.cfg.verbose = verbose;
    }

    /// Set the base address of stack memory map
    fn set_stack_base(&mut self, addr: u64) {
        self.emu.cfg.stack_addr = addr;
    }

    /// when the execution reached a specified amount of steps will spawn an interactive console.
    fn spawn_console_at_pos(&mut self, position: u64) {
        //self.emu.cfg.console = true;
        //self.emu.cfg.console_num = position;
        self.emu.cfg.console_enabled = true;
        self.emu.spawn_console_at(position);
    }

    /// when the execution reached a specified address will spawn an interactive console.
    fn spawn_console_at_addr(&mut self, addr: u64) {
        self.emu.cfg.console2 = true;
        self.emu.cfg.console_addr = addr;
        self.emu.cfg.console_enabled = true;
    }

    /// disable the console spawning.
    fn disable_spawn_console_at_pos(&mut self) {
        self.emu.cfg.console_num = 0;
    }

    /// allow to enable the console if its needed.
    fn enable_console(&mut self) {
        self.emu.cfg.console_enabled = true;
    }

    /// disable the console, to prevent to be spawned in some situations.
    fn disable_console(&mut self) {
        self.emu.cfg.console_enabled = false;
    }

    /// enable the loops counter, this feature slows down the emulation but count the iteration number.
    fn enable_count_loops(&mut self) {
        self.emu.cfg.loops = true;
    }

    /// disable the loops counting system.
    fn disable_count_loops(&mut self) {
        self.emu.cfg.loops = false;
    }

    /// Allow emulating zero-filled code blocks (disables the empty block detector).
    fn allow_empty_code_blocks(&mut self) {
        self.emu.cfg.allow_empty_code_blocks = true;
    }

    /// enable tracing a string on a specified memory address.
    fn enable_trace_string(&mut self, addr: u64) {
        self.emu.cfg.trace_string = true;
        self.emu.cfg.string_addr = addr;
    }

    /// disable the string tracer.
    fn disable_trace_string(&mut self) {
        self.emu.cfg.trace_string = false;
        self.emu.cfg.string_addr = 0;
    }

    /// inspect a memory area by providing a stirng like 'dword ptr [esp + 0x8]'
    fn enable_inspect_sequence(&mut self, seq: &str) {
        self.emu.cfg.inspect = true;
        self.emu.cfg.inspect_seq = seq.to_string();
    }

    fn enable_shellcode_mode(&mut self) {
        self.emu.cfg.shellcode = true;
    }

    /// disable the memory inspector.
    fn disable_inspect_sequence(&mut self) {
        self.emu.cfg.inspect = false;
    }

    /*
    /// give the binary the posibility of connecting remote hosts to get next stage, use it safelly.
    fn enable_endpoint_mode(&mut self) {
        self.emu.cfg.endpoint = true;
    }

    /// disable the endpoint mode.
    fn disable_endpoint_mode(&mut self) {
        self.emu.cfg.endpoint = false;
    }*/

    /// change the default entry point.
    fn set_entry_point(&mut self, addr: u64) {
        self.emu.cfg.entry_point = addr;
    }

    /// rebase the program address.
    fn set_base_address(&mut self, addr: u64) {
        self.emu.cfg.code_base_addr = addr;
    }

    /// enable the stack tracer.
    fn enable_stack_trace(&mut self) {
        self.emu.cfg.stack_trace = true;
    }

    /// disable the stack tracer.
    fn disable_stack_trace(&mut self) {
        self.emu.cfg.stack_trace = false;
    }

    /// test mode use inline assembly to contrast the result of emulation and detect bugs.
    fn enable_test_mode(&mut self) {
        self.emu.cfg.test_mode = true;
    }

    /// disable the test mode.
    fn disable_test_mode(&mut self) {
        self.emu.cfg.test_mode = false;
    }

    /// Enable banzai mode. This mode keep emulating after finding unimplemented instructions or apis.
    fn enable_banzai_mode(&mut self) {
        self.emu.cfg.skip_unimplemented = true;
        self.emu.maps.set_banzai(true);
    }

    /// disable banzai mode.
    fn disable_banzai_mode(&mut self) {
        self.emu.cfg.skip_unimplemented = false;
    }

    /// Add API to banzai.
    fn banzai_add(&mut self, apiname: &str, nparams: i32) {
        self.emu.banzai_add(apiname, nparams);
    }

    /// enable the Control-C handling for spawning console.
    fn enable_ctrlc(&mut self) {
        self.emu.enable_ctrlc();
    }

    /// disable the Control-C handling.
    fn disable_ctrlc(&mut self) {
        self.emu.disable_ctrlc();
    }

    // end of config

    /// It is necessary to load the 32bits or 64bits maps folder for having a realistic memory layout.
    /// The maps can be downloaded from the https://github.com/sha0coder/mwemu
    fn load_maps(&mut self, folder: &str) {
        self.emu.cfg.maps_folder = folder.to_string();
    }

    /// Fetch genuine Windows system DLLs from Microsoft's symbol server and use
    /// them as the maps folder, instead of needing a local copy. `version` is a
    /// friendly name ("win11", "win10", "win2019") or an exact build number
    /// ("26100.7920"). Also sets the build so any DLL the loader needs later is
    /// auto-fetched on demand. Requires network access on first use; results are
    /// cached under maps/winver/.
    ///
    /// Example:
    ///     emu = pymwemu.init_win64()
    ///     emu.load_maps_from_winver("win11")
    ///     emu.load_binary("sample.exe")
    fn load_maps_from_winver(&mut self, version: &str) -> PyResult<()> {
        self.emu
            .set_maps_from_winver(version)
            .map_err(|e| PyValueError::new_err(format!("winver maps fetch failed: {}", e)))
    }

    /// load_binary() already calls init_win32() if its PE or shellcode.
    /// if you dont use load_binary and need the windows simulation
    /// then call this to have peb/teb/ldr/dlls loaded.
    fn init_win32(&mut self) {
        self.emu.init_win32(false, false);
    }

    /// load_binary() already calls init_linux64() if its ELF
    /// if you dont use load_binary and need the linux simulation
    /// then call this to have libc etc loaded.
    fn init_linux64(&mut self, dynamic: bool) {
        self.emu.init_linux64(dynamic);
    }

    /// Load the binary to be emulated.
    fn load_binary(&mut self, filename: &str) {
        self.emu.load_code(filename);
    }

    /// Load code from bytes
    fn load_code_bytes(&mut self, bytes: Vec<u8>) {
        self.emu.load_code_bytes(&bytes);
    }

    /// allocate a buffer on the emulated process address space. (permission defaults to RWX)
    #[pyo3(signature = (name, size, permission=None))]
    fn alloc(&mut self, name: &str, size: u64, permission: Option<PyPermission>) -> PyResult<u64> {
        Ok(self.emu.alloc(name, size, permission.map_or(Permission::READ_WRITE_EXECUTE, |p| p.0)))
    }

    /// allocate at specific address (permission defaults to RWX)
    #[pyo3(signature = (name, addr, size, permission=None))]
    fn alloc_at(&mut self, name: &str, addr: u64, size: u64, permission: Option<PyPermission>) {
        self.emu
            .maps
            .create_map(name, addr, size, permission.map_or(Permission::READ_WRITE_EXECUTE, |p| p.0))
            .expect("pymwemu alloc_at out of memory");
    }

    /// load an aditional blob to the memory layout.
    fn load_map(&mut self, name: &str, filename: &str, base_addr: u64) {
        let map = self
            .emu
            .maps
            .create_map(name, base_addr, 1, Permission::READ_WRITE_EXECUTE)
            .expect("pymwemu load_map out of memory");
        map.load(filename);
    }

    /// link library
    fn link_library(&mut self, filepath: &str) -> PyResult<u64> {
        Ok(self.emu.link_library(filepath))
    }

    /// push a 32bits value to the stack.
    fn stack_push32(&mut self, value: u32) -> PyResult<bool> {
        if self.emu.stack_push32(value) {
            Ok(true)
        } else {
            Err(PyValueError::new_err("pushing error"))
        }
    }

    /// push a 64bits value to the stack.
    fn stack_push64(&mut self, value: u64) -> PyResult<bool> {
        if self.emu.stack_push64(value) {
            Ok(true)
        } else {
            Err(PyValueError::new_err("pushing error"))
        }
    }

    /// pop a 32bits value from the stack.
    fn stack_pop32(&mut self) -> PyResult<u32> {
        match self.emu.stack_pop32(false) {
            Some(v) => Ok(v),
            None => Err(PyValueError::new_err("popping error")),
        }
    }

    /// pop a 64bits value from the stack.
    fn stack_pop64(&mut self) -> PyResult<u64> {
        match self.emu.stack_pop64(false) {
            Some(v) => Ok(v),
            None => Err(PyValueError::new_err("popping error")),
        }
    }

    /// set program counter (works for any arch).
    fn set_pc(&mut self, addr: u64) {
        self.emu.set_pc(addr);
    }

    /// get program counter (works for any arch).
    fn get_pc(&self) -> u64 {
        self.emu.pc()
    }

    /// get stack pointer (works for any arch).
    fn get_sp(&self) -> u64 {
        self.emu.sp()
    }

    /// set stack pointer (works for any arch).
    fn set_sp(&mut self, addr: u64) {
        self.emu.set_sp(addr);
    }

    /// set rip register, if rip point to an api will be emulated.
    fn set_rip(&mut self, addr: u64) -> PyResult<bool> {
        Ok(self.emu.set_rip(addr, false))
    }

    /// set eip register, if eip point to an api will be emulated.
    fn set_eip(&mut self, addr: u64) -> PyResult<bool> {
        Ok(self.emu.set_eip(addr, false))
    }

    /// spawn an interactive console.
    fn spawn_console(&mut self) {
        self.emu.cfg.console_enabled = true;
        Console::spawn_console(&mut self.emu);
    }

    /// disassemble an address.
    fn disassemble(&mut self, addr: u64, amount: u32) -> PyResult<String> {
        Ok(self.emu.disassemble(addr, amount))
    }

    /// stop emulation
    fn stop(&mut self) {
        self.emu.stop();
    }

    /// start emulating the binary after finding the first return.
    fn run_until_return(&mut self) -> PyResult<u64> {
        match self.emu.run_until_ret() {
            Ok(pc) => Ok(pc),
            Err(e) => Err(PyValueError::new_err(e.message)),
        }
    }

    /// emulate a single step, this is slower than run(address) or run(0)
    fn step(&mut self) -> PyResult<bool> {
        Ok(self.emu.step())
    }

    /// Start emulating the binary until reach the provided end_addr.
    /// Use run() with no params for emulating forever. or call32/call64 for calling a function.
    fn run(&mut self, end_addr: Option<u64>) -> PyResult<u64> {
        match self.emu.run(end_addr) {
            Ok(pc) => Ok(pc),
            Err(e) => Err(PyValueError::new_err(e.message)),
        }
    }

    /// Emulate until reaching a specific instruction position (number of instructions).
    fn run_to(&mut self, position: u64) -> PyResult<u64> {
        match self.emu.run_to(position) {
            Ok(pc) => Ok(pc),
            Err(e) => Err(PyValueError::new_err(e.message)),
        }
    }

    /// read the number of instructions emulated since now.
    fn get_position(&mut self) -> PyResult<u64> {
        Ok(self.emu.pos)
    }

    /// call a 32bits function, internally pushes params in reverse order.
    fn call32(&mut self, address: u64, params: Vec<u32>) -> PyResult<u32> {
        match self.emu.call32(address, &params) {
            Ok(pc) => Ok(pc),
            Err(e) => Err(PyValueError::new_err(e.message)),
        }
    }

    /// call a 64bits function, internally pushes params in reverse order.
    fn call64(&mut self, address: u64, params: Vec<u64>) -> PyResult<u64> {
        match self.emu.call64(address, &params) {
            Ok(pc) => Ok(pc),
            Err(e) => Err(PyValueError::new_err(e.message)),
        }
    }

    fn linux_call64(&mut self, address: u64, params: Vec<u64>) -> PyResult<u64> {
        match self.emu.linux_call64(address, &params) {
            Ok(pc) => Ok(pc),
            Err(e) => Err(PyValueError::new_err(e.message)),
        }
    }

    // registers

    /// read register value ie get_reg('rax') or get_reg('x0')
    fn get_reg(&mut self, reg: &str) -> PyResult<u64> {
        if self.emu.cfg.arch.is_aarch64() {
            match self.emu.regs_aarch64().get_by_name(reg) {
                Some(val) => Ok(val),
                None => Err(PyValueError::new_err("invalid aarch64 register name")),
            }
        } else if self.emu.regs().is_reg(reg) {
            Ok(self.emu.regs().get_by_name(reg))
        } else {
            Err(PyValueError::new_err("invalid register name"))
        }
    }

    /// set register value ie  set_reg('rax', 0x123) or set_reg('x0', 0x123), returns previous value.
    fn set_reg(&mut self, reg: &str, value: u64) -> PyResult<u64> {
        if self.emu.cfg.arch.is_aarch64() {
            match self.emu.regs_aarch64().get_by_name(reg) {
                Some(prev) => {
                    self.emu.regs_aarch64_mut().set_by_name(reg, value);
                    Ok(prev)
                }
                None => Err(PyValueError::new_err("invalid aarch64 register name")),
            }
        } else if self.emu.regs().is_reg(reg) {
            let prev = self.emu.regs().get_by_name(reg);
            self.emu.regs_mut().set_by_name(reg, value);
            Ok(prev)
        } else {
            Err(PyValueError::new_err("invalid register name"))
        }
    }

    /// get the value of a xmm register (x86 only).
    fn get_xmm(&mut self, reg: &str) -> PyResult<u128> {
        if self.emu.cfg.arch.is_aarch64() {
            return Err(PyValueError::new_err(
                "xmm registers not available on aarch64",
            ));
        }
        if self.emu.regs().is_xmm_by_name(reg) {
            return Ok(self.emu.regs().get_xmm_by_name(reg));
        }
        Err(PyValueError::new_err("invalid register name"))
    }

    /// set a value to a xmm register (x86 only).
    fn set_xmm(&mut self, reg: &str, value: u128) -> PyResult<u128> {
        if self.emu.cfg.arch.is_aarch64() {
            return Err(PyValueError::new_err(
                "xmm registers not available on aarch64",
            ));
        }
        if self.emu.regs().is_xmm_by_name(reg) {
            let prev = self.emu.regs().get_xmm_by_name(reg);
            self.emu.regs_mut().set_xmm_by_name(reg, value);
            Ok(prev)
        } else {
            Err(PyValueError::new_err("invalid register name"))
        }
    }

    // memory

    /*fn create_map(&mut self,  name:&str) {
        self.emu.maps.create_map(name);
    }*/

    /// write a little endian qword on memory.
    fn write_qword(&mut self, addr: u64, value: u64) -> PyResult<bool> {
        if self.emu.maps.write_qword(addr, value) {
            Ok(true)
        } else {
            Err(PyValueError::new_err("writting on non allocated address"))
        }
    }

    /// write a little endian dword on memory.
    fn write_dword(&mut self, addr: u64, value: u32) -> PyResult<bool> {
        if self.emu.maps.write_dword(addr, value) {
            Ok(true)
        } else {
            Err(PyValueError::new_err("writting on non allocated address"))
        }
    }

    /// write a little endian word on memory.
    fn write_word(&mut self, addr: u64, value: u16) -> PyResult<bool> {
        if self.emu.maps.write_word(addr, value) {
            Ok(true)
        } else {
            Err(PyValueError::new_err("writting on non allocated address"))
        }
    }

    /// write a byte on memory.
    fn write_byte(&mut self, addr: u64, value: u8) -> PyResult<bool> {
        if self.emu.maps.write_byte(addr, value) {
            Ok(true)
        } else {
            Err(PyValueError::new_err("writting on non allocated address"))
        }
    }

    /// read 128bits big endian.
    fn read_128bits_be(&self, addr: u64) -> PyResult<u128> {
        match self.emu.maps.read_128bits_be(addr) {
            Some(v) => Ok(v),
            None => Err(PyValueError::new_err("reading on non allocated address")),
        }
    }

    /// read 128bits little endian.
    fn read_128bits_le(&self, addr: u64) -> PyResult<u128> {
        match self.emu.maps.read_128bits_le(addr) {
            Some(v) => Ok(v),
            None => Err(PyValueError::new_err("reading on non allocated address")),
        }
    }

    /// read little endian qword.
    fn read_qword(&self, addr: u64) -> PyResult<u64> {
        match self.emu.maps.read_qword(addr) {
            Some(v) => Ok(v),
            None => Err(PyValueError::new_err("reading on non allocated address")),
        }
    }

    /// read little endian dword.
    fn read_dword(&self, addr: u64) -> PyResult<u32> {
        match self.emu.maps.read_dword(addr) {
            Some(v) => Ok(v),
            None => Err(PyValueError::new_err("reading on non allocated address")),
        }
    }

    /// read little endian word.
    fn read_word(&self, addr: u64) -> PyResult<u16> {
        match self.emu.maps.read_word(addr) {
            Some(v) => Ok(v),
            None => Err(PyValueError::new_err("reading on non allocated address")),
        }
    }

    /// read a byte from a memory address.
    fn read_byte(&self, addr: u64) -> PyResult<u8> {
        match self.emu.maps.read_byte(addr) {
            Some(v) => Ok(v),
            None => Err(PyValueError::new_err("reading on non allocated address")),
        }
    }

    /// fill a memory chunk starting at `address`, with a specified `amount` of bytes defined in `byte`.
    fn memset(&mut self, addr: u64, byte: u8, amount: usize) {
        self.emu.maps.memset(addr, byte, amount);
    }

    /// get the size of a wide string.
    fn sizeof_wide(&self, unicode_str_ptr: u64) -> PyResult<usize> {
        Ok(self.emu.maps.sizeof_wide(unicode_str_ptr))
    }

    /// write string on memory.
    fn write_string(&mut self, to: u64, _from: &str) {
        self.emu.maps.write_string(to, _from);
    }

    /// write a wide string on memory.
    pub fn write_wide_string(&mut self, to: u64, _from: &str) {
        self.emu.maps.write_wide_string(to, _from);
    }

    /// write a python list of int bytes to the emulator memory.
    pub fn write_buffer(&mut self, to: u64, _from: Vec<u8>) {
        self.emu.maps.write_buffer(to, &_from);
    }

    /// read a buffer &from the emulator memory to a python list of int bytes.
    pub fn read_buffer(&mut self, _from: u64, sz: usize) -> PyResult<Vec<u8>> {
        Ok(self.emu.maps.read_buffer(_from, sz))
    }

    /// write a python list of int bytes to the emulator memory.
    pub fn write_bytes(&mut self, to: u64, _from: Vec<u8>) {
        self.emu.maps.write_buffer(to, &_from);
    }

    /// print all the maps that match a substring of the keyword provided.
    pub fn print_maps_by_keyword(&self, kw: &str) {
        self.emu.maps.print_maps_keyword(kw);
    }

    /// print all the memory maps on the process address space.
    pub fn print_maps(&self) {
        self.emu.maps.print_maps();
    }

    /// get the base address of a given address. Will make an exception if it's invalid address.
    pub fn get_addr_base(&self, addr: u64) -> PyResult<u64> {
        match self.emu.maps.get_addr_base(addr) {
            Some(v) => Ok(v),
            None => Err(PyValueError::new_err("provided address is not allocated")),
        }
    }

    /// this method checks if the given address is allocated or not.
    pub fn is_mapped(&self, addr: u64) -> PyResult<bool> {
        Ok(self.emu.maps.is_mapped(addr))
    }

    /// get the memory map name where is the given address.
    /// Will cause an exception if the address is not allocated.
    pub fn get_addr_name(&self, addr: u64) -> PyResult<String> {
        match self.emu.maps.get_addr_name(addr) {
            Some(v) => Ok(v.to_string()),
            None => Err(PyValueError::new_err(
                "the address doesnt pertain to an allocated block",
            )),
        }
    }

    /// visualize the bytes on the given address.
    pub fn dump_memory(&self, addr: u64) {
        self.emu.maps.dump(addr);
    }

    /// visualize the `amount` of bytes provided on `address`.
    pub fn dump_n(&self, addr: u64, amount: u64) {
        self.emu.maps.dump_n(addr, amount);
    }

    /// visualize a number of qwords on given address.
    pub fn dump_qwords(&self, addr: u64, n: u64) {
        self.emu.maps.dump_qwords(addr, n);
    }

    /// visualize a number of dwords on a given address.
    pub fn dump_dwords(&self, addr: u64, n: u64) {
        self.emu.maps.dump_dwords(addr, n);
    }

    /// read an amount of bytes from an address to a python object.
    pub fn read_bytes(&mut self, addr: u64, sz: usize) -> PyResult<Vec<u8>> {
        Ok(self.emu.maps.read_bytes(addr, sz).to_vec())
    }

    /// read an amount of bytes from an address to a string of spaced hexa bytes.
    pub fn read_string_of_bytes(&mut self, addr: u64, sz: usize) -> PyResult<String> {
        Ok(self.emu.maps.read_string_of_bytes(addr, sz))
    }

    /// read an ascii string from a memory address,
    /// if the address point to a non allocated zone string will be empty.
    pub fn read_string(&self, addr: u64) -> PyResult<String> {
        Ok(self.emu.maps.read_string(addr))
    }

    /// read a wide string from a memory address,
    /// if the address point to a non allocated zone string will be empty.
    pub fn read_wide_string(&self, addr: u64) -> PyResult<String> {
        Ok(self.emu.maps.read_wide_string(addr))
    }

    /// search a substring on a specific memory map name, it will return a list of matched addresses.
    /// if the string is not found, it will return an empty list.
    pub fn search_string(&self, kw: &str, map_name: &str) -> PyResult<Vec<u64>> {
        match self.emu.maps.search_string(kw, map_name) {
            Some(v) => Ok(v),
            None => Ok(Vec::new()),
        }
    }

    /// write on emulators memory a spaced hexa bytes
    pub fn write_spaced_bytes(&mut self, addr: u64, spaced_hex_bytes: &str) -> PyResult<bool> {
        if self.emu.maps.write_spaced_bytes(addr, spaced_hex_bytes) {
            Ok(true)
        } else {
            Err(PyValueError::new_err(
                "couldnt write the bytes on that address",
            ))
        }
    }

    /// search one occurence of a spaced hex bytes from a specific address, will return zero if it's not found.
    pub fn search_spaced_bytes_from(&self, saddr: u64, sbs: &str) -> PyResult<u64> {
        Ok(self.emu.maps.search_spaced_bytes_from(sbs, saddr))
    }

    /// search one occcurence of a spaced hex bytes from an especific address backward,
    /// will return zero if it's not found.
    pub fn search_spaced_bytes_from_bw(&self, saddr: u64, sbs: &str) -> PyResult<u64> {
        Ok(self.emu.maps.search_spaced_bytes_from_bw(sbs, saddr))
    }

    /// search spaced hex bytes string on specific map using its map name,
    /// will return a list with the addresses found if there are matches,
    /// otherwise the list will be empty.
    pub fn search_spaced_bytes(&self, sbs: &str, map_name: &str) -> PyResult<Vec<u64>> {
        Ok(self.emu.maps.search_spaced_bytes(sbs, map_name))
    }

    /// search spaced hex bytes string on all the memory layout,
    /// will return a list with the addresses found if there are matches,
    /// otherwise the list will be empty.
    pub fn search_spaced_bytes_in_all(&self, sbs: &str) -> PyResult<Vec<u64>> {
        Ok(self.emu.maps.search_spaced_bytes_in_all(sbs))
    }

    /// Search a substring in all the memory layout except on libs, will print the results.
    /// In the future will return a list with results instead of printing.
    pub fn search_string_in_all(&self, kw: String) {
        self.emu.maps.search_string_in_all(kw);
    }

    /// search a bytes object on specific map, will return a list with matched addresses if there are any.
    pub fn search_bytes(&self, bkw: Vec<u8>, map_name: &str) -> PyResult<Vec<u64>> {
        Ok(self.emu.maps.search_bytes(bkw, map_name))
    }

    /// show the total allocated memory.
    pub fn allocated_size(&self) -> PyResult<usize> {
        Ok(self.emu.maps.size())
    }

    /// show if there are memory blocks overlaping eachother.
    pub fn memory_overlaps(&self, addr: u64, sz: u64) -> PyResult<bool> {
        Ok(self.emu.maps.overlaps(addr, sz))
    }

    /// show all the memory blocks allocated during the emulation.
    pub fn show_allocs(&self) {
        self.emu.maps.show_allocs();
    }

    /// free a memory map by its name
    pub fn free(&mut self, name: &str) {
        self.emu.maps.free(name);
    }

    /// basic allocator, it looks for a free block of given size,
    /// it only returns the address if its possible, but dont really allocates,
    /// just find the address, you have to load to that address something.
    /// use alloc() method instead if possible.
    pub fn memory_alloc(&self, sz: u64) -> PyResult<u64> {
        match self.emu.maps.alloc(sz) {
            Some(addr) => Ok(addr),
            None => Err(PyValueError::new_err("couldnt found a space of that size")),
        }
    }

    /// Save all memory blocks allocated during emulation to disk.
    /// Provide a folder where every alloc will be a file.
    pub fn save_all_allocs(&mut self, path: String) {
        self.emu.maps.save_all_allocs(path);
    }

    /// save a chunk of memory to disk.
    pub fn save(&mut self, addr: u64, size: u64, filename: String) {
        self.emu.maps.save(addr, size, filename);
    }

    /// perform a memory test to see overlapps or other possible problems.
    pub fn mem_test(&self) -> PyResult<bool> {
        Ok(self.emu.maps.mem_test())
    }

    /// breakpoints
    /// show breakpoints
    pub fn bp_show(&self) {
        self.emu.bp.show();
    }

    /// clear all the breakpoints
    pub fn bp_clear_all(&mut self) {
        self.emu.bp.clear_bp();
    }

    /// set breakpoint on an address
    pub fn bp_set_addr(&mut self, addr: u64) {
        self.emu.bp.add_bp(addr);
    }

    /// get the current address breakpoint
    pub fn bp_get_addr(&self) -> PyResult<Vec<u64>> {
        Ok(self.emu.bp.addr.clone())
    }

    /// set breakpoint on a instruction counter
    pub fn bp_set_inst(&mut self, ins: u64) {
        self.emu.bp.add_bp_instruction(ins);
    }

    /// get breakpoint on a instrunction counter
    pub fn bp_get_inst(&self) -> PyResult<Vec<u64>> {
        Ok(self.emu.bp.instruction.clone())
    }

    /// set a memory breakpoint on read
    pub fn bp_set_mem_read(&mut self, addr: u64) {
        self.emu.bp.add_bp_mem_read(addr);
    }

    /// get the memory breakpoint on read
    pub fn bp_get_mem_read(&self) -> PyResult<Vec<u64>> {
        Ok(self.emu.bp.mem_read_addr.clone())
    }

    /// set a memory breakpoint on write
    pub fn bp_set_mem_write(&mut self, addr: u64) {
        self.emu.bp.add_bp_mem_write(addr);
    }

    /// get the memory breakpoint on write
    pub fn bp_get_mem_write(&self) -> PyResult<Vec<u64>> {
        Ok(self.emu.bp.mem_write_addr.clone())
    }

    /// handle winapi address
    pub fn handle_winapi(&mut self, addr: u64) {
        self.emu.handle_winapi(addr);
    }

    /// emulate until next winapi call
    pub fn run_until_apicall(&mut self) -> PyResult<(u64, String)> {
        self.emu.skip_apicall = true;
        self.emu.is_break_on_api = true;
        // run until api
        let _ = self.emu.run(None);
        match self.emu.its_apicall {
            Some(addr) => {
                self.emu.skip_apicall = false;
                let name = self.emu.api_addr_to_name(addr);
                let new_pc = self.emu.pc() + self.emu.last_instruction_size as u64;
                self.emu.set_pc(new_pc);
                return Ok((addr, name));
            }

            _ => Err(PyException::new_err(
                "breakpoint on apicall fail because there isn't any address return",
            )),
        }
    }


    // --- Serialization ---
    /// serialize the whole emulator state to a bytes object, which can be saved to disk for example
    pub fn serialize(&self) -> PyResult<Vec<u8>> {
        Ok(libmwemu::serialization::Serialization::serialize(&self.emu))
    }

    /// serialize the whole emulator state to a file, which can be loaded later with deserialize_dump
    pub fn dump_to_file(&self, filename: &str) {
        libmwemu::serialization::Serialization::dump_to_file(&self.emu, filename);
    }

    /// serialize the whole emulator state to a minidump file, which can be loaded later with load_from_minidump
    pub fn dump_to_minidump(&self, filename: &str) {
        let _ = libmwemu::serialization::Serialization::dump_to_minidump(&self.emu, filename);
    }

    pub fn dump(&self, filename: &str) {
        libmwemu::serialization::Serialization::dump(&self.emu, filename);
    }

    // -- CFG getters/setters --
    // execution limits
    #[setter]
    pub fn set_max_instructions(&mut self, max: Option<u64>) {
        self.emu.cfg.max_instructions = max;
    }

    #[getter]
    pub fn get_max_instructions(&self) -> Option<u64> {
        self.emu.cfg.max_instructions
    }

    #[setter]
    pub fn set_timeout_secs(&mut self, secs: Option<f64>) {
        self.emu.cfg.timeout_secs = secs;
    }

    #[getter]
    pub fn get_timeout_secs(&self) -> Option<f64> {
        self.emu.cfg.timeout_secs
    }

    #[setter]
    pub fn set_max_faults(&mut self, faults: Option<u32>) {
        self.emu.cfg.max_faults = faults;
    }

    #[getter]
    pub fn get_max_faults(&self) -> Option<u32> {
        self.emu.cfg.max_faults
    }

    #[setter]
    pub fn set_exit_position(&mut self, pos: u64) {
        self.emu.cfg.exit_position = pos;
    }

    #[getter]
    pub fn get_exit_position(&self) -> u64 {
        self.emu.cfg.exit_position
    }

    #[setter]
    pub fn set_dump_on_exit(&mut self, dump: bool) {
        self.emu.cfg.dump_on_exit = dump;
    }

    #[getter]
    pub fn get_dump_on_exit(&self) -> bool {
        self.emu.cfg.dump_on_exit
    }

    #[setter]
    pub fn set_dump_filename(&mut self, filename: Option<String>) {
        self.emu.cfg.dump_filename = filename;
    }

    #[getter]
    pub fn get_dump_filename(&self) -> Option<String> {
        self.emu.cfg.dump_filename.clone()
    }

    // tracing
    pub fn open_trace_file(&mut self)
    {
        self.emu.open_trace_file();
    }

    #[setter]
    pub fn set_trace_filename(&mut self, filename: Option<String>) {
        self.emu.cfg.trace_filename = filename;
    }

    #[getter]
    pub fn get_trace_filename(&self) -> Option<String> {
        self.emu.cfg.trace_filename.clone()
    }

    #[setter]
    pub fn set_trace_start(&mut self, start: u64) {
        self.emu.cfg.trace_start = start;
    }

    #[getter]
    pub fn get_trace_start(&self) -> u64 {
        self.emu.cfg.trace_start
    }

    #[setter]
    pub fn set_trace_calls(&mut self, calls: bool) {
        self.emu.cfg.trace_calls = calls;
    }

    #[getter]
    pub fn get_trace_calls(&self) -> bool {
        self.emu.cfg.trace_calls
    }

    #[setter]
    pub fn set_trace_flags(&mut self, flags: bool) {
        self.emu.cfg.trace_flags = flags;
    }

    // Environnement identity
    #[setter]
    pub fn set_module_name(&mut self, name: String) {
        self.emu.cfg.module_name = name;
    }

    #[getter]
    pub fn get_module_name(&self) -> String {
        self.emu.cfg.module_name.clone()
    }

    #[setter]
    pub fn set_exe_name(&mut self, name: String) {
        self.emu.cfg.exe_name = name;
    }

    #[getter]
    pub fn get_exe_name(&self) -> String {
        self.emu.cfg.exe_name.clone()
    }

    #[setter]
    pub fn set_user_name(&mut self, name: String) {
        self.emu.cfg.user_name = name;
    }

    #[getter]
    pub fn get_user_name(&self) -> String {
        self.emu.cfg.user_name.clone()
    }

    #[setter]
    pub fn set_host_name(&mut self, name: String) {
        self.emu.cfg.host_name = name;
    }

    #[setter]
    pub fn set_temp_path(&mut self, path: String) {
        self.emu.cfg.temp_path = path;
    }

    #[getter]
    pub fn get_temp_path(&self) -> String {
        self.emu.cfg.temp_path.clone()
    }

    #[setter]
    pub fn set_cwd_path(&mut self, path: String) {
        self.emu.cfg.cwd_path = path;
    }

    #[getter]
    pub fn get_cwd_path(&self) -> String {
        self.emu.cfg.cwd_path.clone()
    }

    #[setter]
    pub fn set_windows_directory(&mut self, path: String) {
        self.emu.cfg.windows_directory = path;
    }

    #[getter]
    pub fn get_windows_directory(&self) -> String {
        self.emu.cfg.windows_directory.clone()
    }

    #[setter]
    pub fn set_system_directory(&mut self, path: String) {
        self.emu.cfg.system_directory = path;
    }

    #[getter]
    pub fn get_system_directory(&self) -> String {
        self.emu.cfg.system_directory.clone()
    }

    #[setter]
    pub fn set_emulate_winapi(&mut self, emulate: bool) {
        self.emu.cfg.emulate_winapi = emulate;
    }

    #[getter]
    pub fn get_emulate_winapi(&self) -> bool {
        self.emu.cfg.emulate_winapi
    }

    #[setter]
    pub fn set_short_circuit_sleep(&mut self, short_circuit: bool) {
        self.emu.cfg.short_circuit_sleep = short_circuit;
    }

    #[getter]
    pub fn get_short_circuit_sleep(&self) -> bool {
        self.emu.cfg.short_circuit_sleep
    }

    #[setter]
    pub fn set_heap_alloc_min_size(&mut self, size: u64) {
        self.emu.cfg.heap_alloc_min_size = size;
    }

    #[getter]
    pub fn get_heap_alloc_min_size(&self) -> u64 {
        self.emu.cfg.heap_alloc_min_size
    }

    #[setter]
    pub fn set_heap_free_soft(&mut self, free: bool) {
        self.emu.cfg.heap_free_soft = free;
    }

    #[getter]
    pub fn get_heap_free_soft(&self) -> bool {
        self.emu.cfg.heap_free_soft
    }

    #[setter]
    pub fn set_ssdt_use_ldr_initialize_thunk(&mut self, use_ldr: bool) {
        self.emu.cfg.ssdt_use_ldr_initialize_thunk = use_ldr;
    }

    #[getter]
    pub fn get_ssdt_use_ldr_initialize_thunk(&self) -> bool {
        self.emu.cfg.ssdt_use_ldr_initialize_thunk
    }

    // call stack
    pub fn get_call_stack(&self) -> Vec<(u64, u64)> {
        self.emu.call_stack().clone()
    }


    // enable-threading
    pub fn enable_threading(&mut self, enable: bool) {
        self.emu.enable_threading(enable);
    }
}

// --- Serialization ---

// The libmwemu deserialization/load routines (and the PE parser they re-run on
// the embedded `raw` bytes) are full of `.unwrap()`/`panic!` and take no
// fallible path. Feeding them a corrupt, truncated or hostile blob therefore
// aborts the whole process instead of raising a catchable Python error. Since
// these inputs are attacker-controllable in a malware-analysis workflow (state
// dumps / minidumps from untrusted samples or sandboxes), we contain the panic
// here at the FFI boundary and surface it as a normal `ValueError`.
//
// This is the minimal containment; the deeper fix is to make the PE parser
// bounds-check and return `Result` upstream in libmwemu.
// TODO: harden PE parser bounds (PE64/PE32::load_from_raw) to remove panics at the source.
fn catch_deserialize<F>(what: &str, f: F) -> PyResult<Emu>
where
    F: FnOnce() -> Emu,
{
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).map_err(|_| {
        PyValueError::new_err(format!(
            "failed to {what}: input is corrupt, truncated, or malformed"
        ))
    })
}

#[gen_stub_pyfunction(module="pymwemu._pymwemu")]
#[pyfunction]
/// deserialize the emulator state from a bytes object, which can be loaded from disk for example
pub fn deserialize(data: Vec<u8>) -> PyResult<Emu> {
    catch_deserialize("deserialize emulator state", || Emu {
        emu: libmwemu::serialization::Serialization::deserialize(&data),
    })
}
#[gen_stub_pyfunction(module="pymwemu._pymwemu")]
#[pyfunction]
/// deserialize the emulator state from a minidump file, which can be dumped with dump_to_minidump
pub fn load_from_minidump(filename: &str) -> PyResult<Emu> {
    catch_deserialize("load emulator state from minidump", || Emu {
        emu: libmwemu::serialization::Serialization::load_from_minidump(filename),
    })
}

#[gen_stub_pyfunction(module="pymwemu._pymwemu")]
#[pyfunction]
/// deserialize the emulator state from a file, which can be dumped with dump_to_file
pub fn load_from_file(filename: &str) -> PyResult<Emu> {
    catch_deserialize("load emulator state from file", || Emu {
        emu: libmwemu::serialization::Serialization::load_from_file(filename),
    })
}

#[gen_stub_pyfunction(module="pymwemu._pymwemu")]
#[pyfunction]
/// deserialize the emulator state from a file, which can be dumped with dump
pub fn load(filename: &str) -> PyResult<Emu> {
    catch_deserialize("load emulator state", || Emu {
        emu: libmwemu::serialization::Serialization::load(filename),
    })
}



#[gen_stub_pyfunction(module="pymwemu._pymwemu")]
#[pyfunction]
fn init32() -> PyResult<Emu> {
    let mut emu = Emu { emu: emu32() };
    emu.emu.cfg.console_enabled = false;
    emu.emu.cfg.verbose = 0;
    emu.emu.cfg.shellcode = false;

    Ok(emu)
}

#[gen_stub_pyfunction(module = "pymwemu._pymwemu")]
#[pyfunction]
fn init64() -> PyResult<Emu> {
    let mut emu = Emu { emu: emu64() };
    emu.emu.cfg.console_enabled = false;
    emu.emu.cfg.verbose = 0;
    emu.emu.cfg.shellcode = false;

    Ok(emu)
}

#[gen_stub_pyfunction(module = "pymwemu._pymwemu")]
#[pyfunction]
fn init_aarch64() -> PyResult<Emu> {
    let mut emu = Emu { emu: emu_aarch64() };
    emu.emu.cfg.console_enabled = false;
    emu.emu.cfg.verbose = 0;
    emu.emu.cfg.shellcode = false;

    Ok(emu)
}

#[pymodule]
fn _pymwemu(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    // Filter `goblin=warn` to drop the LoadCommand / Mach-o header / Ctx
    // spam goblin emits via `debug!()` while parsing Mach-O / PE / ELF —
    // the mwemu CLI does the equivalent via a fast_log Filter.
    env_logger::Builder::from_env(Env::default().default_filter_or("trace,goblin=warn"))
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();
    log::info!("Initialized logging");

    // we register Emu inside the _pymwemu module, which would re-export as pymwemu.Emu
    m.add_class::<Emu>()?;
    m.add_function(wrap_pyfunction!(init32, m)?)?;
    m.add_function(wrap_pyfunction!(init64, m)?)?;
    m.add_function(wrap_pyfunction!(init_aarch64, m)?)?;


    // Serialization functions
    m.add_function(wrap_pyfunction!(deserialize, m)?)?;
    m.add_function(wrap_pyfunction!(load_from_minidump, m)?)?;
    m.add_function(wrap_pyfunction!(load_from_file, m)?)?;
    m.add_function(wrap_pyfunction!(load, m)?)?;

    m.add_class::<PyPermission>()?;

    Ok(())
}
//

// Re-export the module members of _pymwemu to pymwemu
pyo3_stub_gen::reexport_module_members!("pymwemu" from "pymwemu._pymwemu");

//  Define a stub info gatherer, to generate the .pyi
define_stub_info_gatherer!(stub_gen);

#[cfg(test)]
mod tests {
    use super::*;

    // A corrupt/hostile blob must surface as a catchable Python error instead of
    // aborting the process (the previous behaviour, via libmwemu's `.unwrap()`s).
    #[test]
    fn deserialize_rejects_garbage_without_panicking() {
        let garbage = vec![0xFFu8; 64];
        let result = deserialize(garbage);
        assert!(
            result.is_err(),
            "corrupt blob must return Err, not a valid Emu"
        );
    }

    #[test]
    fn deserialize_rejects_empty_input() {
        assert!(deserialize(Vec::new()).is_err());
    }

    #[test]
    fn load_from_file_rejects_missing_file() {
        let result = load_from_file("/nonexistent/path/should/not/exist.bin");
        assert!(result.is_err(), "missing file must return Err, not abort");
    }
}

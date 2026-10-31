use crate::emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::exception::types;

/// Interrupt hook: `(emu, interrupt_number, ip)`. Return false to skip default handling.
pub type TypeHookOnInterrupt = Box<dyn FnMut(&mut emu::Emu, u64, u64) -> bool>;
/// Exception hook: `(emu, ip, exception_type)`. Return false to suppress default handling.
pub type TypeHookOnException = Box<dyn FnMut(&mut emu::Emu, u64, types::ExceptionType) -> bool>;
/// Pre-read hook: `(emu, ip, addr, size)`. Fires before every memory read.
pub type TypeHookOnMemoryRead = Box<dyn FnMut(&mut emu::Emu, u64, u64, u32)>;
/// Pre-write hook: `(emu, ip, addr, size, value)`. Return the value to actually write.
pub type TypeHookOnMemoryWrite = Box<dyn FnMut(&mut emu::Emu, u64, u64, u32, u128) -> u128>;

// [BREAKING API CHANGE] returning false will skip the handling of the instruction
// Changed from &iced_x86::Instruction to &DecodedInstruction for arch parity.
// x86 hook users call ins.as_x86() to get the familiar type.
pub type TypeHookOnPreInstruction =
    Box<dyn FnMut(&mut emu::Emu, u64, &DecodedInstruction, usize) -> bool>;

pub type TypeHookOnPostInstruction =
    Box<dyn FnMut(&mut emu::Emu, u64, &DecodedInstruction, usize, bool)>;
pub type TypeHookOnWinApiCall = Box<dyn FnMut(&mut emu::Emu, u64, u64) -> bool>;
// (emu, syscall_nr): fires on every syscall/SVC across all OSes, before the
// internal dispatch. `syscall_nr` is the raw number the program invoked (rax /
// x8 / x16 / eax depending on arch); on Windows you can map it to the canonical
// number via `emu.syscall_number_map`. Return false if the hook fully handled
// the syscall (it set the return register itself) to skip the default handler.
pub type TypeHookOnSyscall = Box<dyn FnMut(&mut emu::Emu, u64) -> bool>;

/// Callback hooks for intercepting emulation events (interrupts, exceptions, memory, instructions, API calls).
pub struct Hooks {
    pub hook_on_interrupt: Option<TypeHookOnInterrupt>,
    pub hook_on_exception: Option<TypeHookOnException>,
    pub hook_on_memory_read: Option<TypeHookOnMemoryRead>,
    pub hook_on_memory_write: Option<TypeHookOnMemoryWrite>,
    pub hook_on_pre_instruction: Option<TypeHookOnPreInstruction>,
    pub hook_on_post_instruction: Option<TypeHookOnPostInstruction>,
    pub hook_on_winapi_call: Option<TypeHookOnWinApiCall>,
    pub hook_on_syscall: Option<TypeHookOnSyscall>,
}

impl Default for Hooks {
    fn default() -> Self {
        Self::new()
    }
}

impl Hooks {
    /// Create a new Hooks instance with all hooks disabled.
    pub fn new() -> Hooks {
        Hooks {
            hook_on_interrupt: None,
            hook_on_exception: None,
            hook_on_memory_read: None,
            hook_on_memory_write: None,
            hook_on_pre_instruction: None,
            hook_on_post_instruction: None,
            hook_on_winapi_call: None,
            hook_on_syscall: None,
        }
    }

    /// Register an interrupt hook. Return false from the callback to skip default handling.
    pub fn on_interrupt(&mut self, hook: impl FnMut(&mut emu::Emu, u64, u64) -> bool + 'static) {
        self.hook_on_interrupt = Some(Box::new(hook));
    }

    /// Remove the interrupt hook.
    pub fn disable_interrupt(&mut self) {
        self.hook_on_interrupt = None;
    }

    /// Register an exception hook. Return false to suppress default handling.
    pub fn on_exception(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, types::ExceptionType) -> bool + 'static,
    ) {
        self.hook_on_exception = Some(Box::new(hook));
    }

    /// Remove the exception hook.
    pub fn disable_exception(&mut self) {
        self.hook_on_exception = None;
    }

    /// Register a pre-read hook that fires before every memory read.
    pub fn on_memory_read(&mut self, hook: impl FnMut(&mut emu::Emu, u64, u64, u32) + 'static) {
        self.hook_on_memory_read = Some(Box::new(hook));
    }

    /// Remove the memory-read hook.
    pub fn disable_memory_read(&mut self) {
        self.hook_on_memory_read = None;
    }

    /// Register a pre-write hook. Return the value to actually write.
    pub fn on_memory_write(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, u64, u32, u128) -> u128 + 'static,
    ) {
        self.hook_on_memory_write = Some(Box::new(hook));
    }

    /// Remove the memory-write hook.
    pub fn disable_memory_write(&mut self) {
        self.hook_on_memory_write = None;
    }

    /// Register a pre-instruction hook. Return false to skip the instruction.
    pub fn on_pre_instruction(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, &DecodedInstruction, usize) -> bool + 'static,
    ) {
        self.hook_on_pre_instruction = Some(Box::new(hook));
    }

    /// Remove the pre-instruction hook.
    pub fn disable_pre_instruction(&mut self) {
        self.hook_on_pre_instruction = None;
    }

    /// Register a post-instruction hook, called after each instruction executes.
    pub fn on_post_instruction(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, &DecodedInstruction, usize, bool) + 'static,
    ) {
        self.hook_on_post_instruction = Some(Box::new(hook));
    }

    /// Remove the post-instruction hook.
    pub fn disable_post_instruction(&mut self) {
        self.hook_on_post_instruction = None;
    }

    /// Register a WinAPI call hook. Return false to skip the API handler.
    pub fn on_winapi_call(&mut self, hook: impl FnMut(&mut emu::Emu, u64, u64) -> bool + 'static) {
        self.hook_on_winapi_call = Some(Box::new(hook));
    }

    /// Remove the WinAPI call hook.
    pub fn disable_winapi_call(&mut self) {
        self.hook_on_winapi_call = None;
    }

    /// Register a syscall hook. Return false to skip the default syscall handler.
    pub fn on_syscall(&mut self, hook: impl FnMut(&mut emu::Emu, u64) -> bool + 'static) {
        self.hook_on_syscall = Some(Box::new(hook));
    }

    /// Remove the syscall hook.
    pub fn disable_syscall(&mut self) {
        self.hook_on_syscall = None;
    }
}

impl emu::Emu {
    /// Run the installed `on_syscall` hook (if any) for `nr`. Returns `false`
    /// when the hook reports it fully handled the syscall and the default
    /// dispatch should be skipped. Uses take/restore so the hook can borrow the
    /// emulator mutably while it is stored inside it.
    pub fn call_syscall_hook(&mut self, nr: u64) -> bool {
        if let Some(mut hook_fn) = self.hooks.hook_on_syscall.take() {
            let proceed = hook_fn(self, nr);
            self.hooks.hook_on_syscall = Some(hook_fn);
            proceed
        } else {
            true
        }
    }

    // --- Hook registration convenience methods ---

    /// Register an interrupt hook. Return false from the callback to skip default handling.
    pub fn set_hook_on_interrupt(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, u64) -> bool + 'static,
    ) {
        self.hooks.on_interrupt(hook);
    }

    /// Register an exception hook. Return false to suppress default handling.
    pub fn set_hook_on_exception(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, types::ExceptionType) -> bool + 'static,
    ) {
        self.hooks.on_exception(hook);
    }

    /// Register a pre-read hook that fires before every memory read.
    pub fn set_hook_on_memory_read(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, u64, u32) + 'static,
    ) {
        self.hooks.on_memory_read(hook);
    }

    /// Register a pre-write hook. Return the value to actually write.
    pub fn set_hook_on_memory_write(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, u64, u32, u128) -> u128 + 'static,
    ) {
        self.hooks.on_memory_write(hook);
    }

    /// Register a pre-instruction hook. Return false to skip the instruction.
    pub fn set_hook_on_pre_instruction(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, &DecodedInstruction, usize) -> bool + 'static,
    ) {
        self.hooks.on_pre_instruction(hook);
    }

    /// Register a post-instruction hook, called after each instruction executes.
    pub fn set_hook_on_post_instruction(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, &DecodedInstruction, usize, bool) + 'static,
    ) {
        self.hooks.on_post_instruction(hook);
    }

    /// Register a WinAPI call hook. Return false to skip the API handler.
    pub fn set_hook_on_winapi_call(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, u64) -> bool + 'static,
    ) {
        self.hooks.on_winapi_call(hook);
    }

    /// Register a syscall hook. Return false to skip the default syscall handler.
    pub fn set_hook_on_syscall(&mut self, hook: impl FnMut(&mut emu::Emu, u64) -> bool + 'static) {
        self.hooks.on_syscall(hook);
    }

    /// Remove all installed hooks.
    pub fn clear_hooks(&mut self) {
        self.hooks = Hooks::new();
    }
}

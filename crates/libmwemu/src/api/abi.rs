use crate::emu::Emu;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApiAbi {
    Aarch64,
    X86_64,
}

impl ApiAbi {
    #[inline]
    pub fn from_emu(emu: &Emu) -> Self {
        if emu.cfg.arch.is_x64() {
            Self::X86_64
        } else {
            Self::Aarch64
        }
    }

    #[inline]
    pub fn arg(self, emu: &Emu, idx: usize) -> u64 {
        match self {
            Self::Aarch64 => {
                let regs = emu.regs_aarch64();
                regs.x.get(idx).copied().unwrap_or_else(|| {
                    unreachable!("AArch64 API arg{} is out of range", idx);
                })
            }
            Self::X86_64 => match idx {
                0 => {
                    let regs = emu.regs();
                    regs.rdi
                }
                1 => {
                    let regs = emu.regs();
                    regs.rsi
                }
                2 => {
                    let regs = emu.regs();
                    regs.rdx
                }
                3 => {
                    let regs = emu.regs();
                    regs.rcx
                }
                4 => {
                    let regs = emu.regs();
                    regs.r8
                }
                5 => {
                    let regs = emu.regs();
                    regs.r9
                }
                // SysV AMD64 passes args 0-5 in registers and the rest on the
                // stack. The kernel gateway has already popped the return
                // address, so rsp points at the first stack argument (arg6);
                // arg N is at [rsp + (N-6)*8]. Needed by wide kernel APIs such
                // as usb_control_msg (data=arg6, size=arg7).
                n => {
                    let rsp = emu.regs().rsp;
                    let off = (n as u64 - 6) * 8;
                    emu.maps.read_qword(rsp + off).unwrap_or(0)
                }
            },
        }
    }

    #[inline]
    pub fn set_ret(self, emu: &mut Emu, value: u64) {
        match self {
            Self::Aarch64 => emu.regs_aarch64_mut().x[0] = value,
            Self::X86_64 => emu.regs_mut().rax = value,
        }
    }
}

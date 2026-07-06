use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    // guloader checks bit31 which is if its hipervisor with command
    // https://c9x.me/x86/html/file_module_x86_id_45.html
    // TODO: implement 0x40000000 -> get the virtualization vendor

    if emu.cfg.verbose >= 1 {
        log::trace!(
            "\tcpuid input value: 0x{:x}, 0x{:x}",
            emu.regs().rax,
            emu.regs().rcx
        );
    }

    match emu.regs().rax {
        0x00 => {
            emu.regs_mut().rax = 0x16;
            emu.regs_mut().rbx = 0x756e6547;
            emu.regs_mut().rcx = 0x6c65746e;
            emu.regs_mut().rdx = 0x49656e69;
        }
        0x01 => {
            emu.regs_mut().rax = 0x906ed; // Version Information (Type, Family, Model, and Stepping ID)
            emu.regs_mut().rbx = 0x5100800;
            emu.regs_mut().rcx = 0x7ffafbbf;
            emu.regs_mut().rdx = 0xbfebfbff; // feature
            set_bit!(emu.regs_mut().rdx, 0, 1); // FPU = true
            set_bit!(emu.regs_mut().rdx, 23, 1); // MMX = true
            set_bit!(emu.regs_mut().rdx, 25, 1); // SSE = true
            set_bit!(emu.regs_mut().rdx, 26, 1); // SSE2 = true
        }
        0x02 => {
            emu.regs_mut().rax = 0x76036301;
            emu.regs_mut().rbx = 0xf0b5ff;
            emu.regs_mut().rcx = 0;
            emu.regs_mut().rdx = 0xc30000;
        }
        0x03 => {
            emu.regs_mut().rax = 0;
            emu.regs_mut().rbx = 0;
            emu.regs_mut().rcx = 0;
            emu.regs_mut().rdx = 0;
        }
        0x04 => {
            emu.regs_mut().rax = 0;
            emu.regs_mut().rbx = 0x1c0003f;
            emu.regs_mut().rcx = 0x3f;
            emu.regs_mut().rdx = 0;
        }
        0x05 => {
            emu.regs_mut().rax = 0x40;
            emu.regs_mut().rbx = 0x40;
            emu.regs_mut().rcx = 3;
            emu.regs_mut().rdx = 0x11142120;
        }
        0x06 => {
            emu.regs_mut().rax = 0x27f7;
            emu.regs_mut().rbx = 2;
            emu.regs_mut().rcx = 9;
            emu.regs_mut().rdx = 0;
        }
        0x0d => {
            match emu.regs().rcx {
                1 => {
                    emu.regs_mut().rax = 0xf;
                    emu.regs_mut().rbx = 0x3c0;
                    emu.regs_mut().rcx = 0x100;
                    emu.regs_mut().rdx = 0;
                }
                0 => {
                    emu.regs_mut().rax = 0x1f;
                    emu.regs_mut().rbx = 0x440;
                    emu.regs_mut().rcx = 0x440;
                    emu.regs_mut().rdx = 0;
                }
                2 => {
                    emu.regs_mut().rax = 0x100;
                    emu.regs_mut().rbx = 0x240;
                    emu.regs_mut().rcx = 0;
                    emu.regs_mut().rdx = 0;
                }
                3 => {
                    emu.regs_mut().rax = 0x40;
                    emu.regs_mut().rbx = 0x3c0;
                    emu.regs_mut().rcx = 0;
                    emu.regs_mut().rdx = 0;
                }
                5..=7 => {
                    emu.regs_mut().rax = 0;
                    emu.regs_mut().rbx = 0;
                    emu.regs_mut().rcx = 0;
                    emu.regs_mut().rdx = 0;
                }
                _ => {
                    emu.regs_mut().rax = 0x1f; //0x1f
                    emu.regs_mut().rbx = 0x440; //0x3c0; // 0x440
                    emu.regs_mut().rcx = 0x440; //0x100; // 0x440
                    emu.regs_mut().rdx = 0;
                }
            }
        }
        0x07..=0x6d => {
            emu.regs_mut().rax = 0;
            emu.regs_mut().rbx = 0x29c67af;
            emu.regs_mut().rcx = 0x40000000;
            emu.regs_mut().rdx = 0xbc000600;
        }
        0x6e => {
            emu.regs_mut().rax = 0x960;
            emu.regs_mut().rbx = 0x1388;
            emu.regs_mut().rcx = 0x64;
            emu.regs_mut().rdx = 0;
        }
        0x80000000 => {
            emu.regs_mut().rax = 0x80000008;
            emu.regs_mut().rbx = 0;
            emu.regs_mut().rcx = 0;
            emu.regs_mut().rdx = 0;
        }
        0x80000001 => {
            emu.regs_mut().rax = 0;
            emu.regs_mut().rbx = 0;
            emu.regs_mut().rcx = 0x121;
            emu.regs_mut().rdx = 0x2c100800;
        }
        0x80000007 => {
            emu.regs_mut().rax = 0;
            emu.regs_mut().rbx = 0;
            emu.regs_mut().rcx = 0;
            emu.regs_mut().rdx = 0x100;
        }
        0x80000008 => {
            emu.regs_mut().rax = 0x3027;
            emu.regs_mut().rbx = 0;
            emu.regs_mut().rcx = 0;
            emu.regs_mut().rdx = 0; //0x100;
        }
        _ => {
            log::trace!("unimplemented cpuid call 0x{:x}", emu.regs().rax);
            return false;
        }
    }
    true
}

use crate::emu::Emu;
use crate::serialization;
use crate::winapi::winapi32;
use crate::winapi::winapi64;
use crate::windows::peb::peb32;
use crate::windows::peb::peb64;
use crate::windows::structures;
use std::io::Write;
use std::num::ParseIntError;
use std::sync::atomic;

// if the user types "r2 0x123" will execute radare2
use crate::maps::mem64::Permission;
use std::fs;
use std::io;
use std::process::{Command, Stdio};

pub struct Console {}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

impl Console {
    pub fn new() -> Console {
        log::trace!("--- console ---");
        Console {}
    }

    pub fn print(&self, msg: &str) {
        log::logger().flush();
        print!("{}", msg);
        std::io::stdout().flush().unwrap();
    }

    pub fn cmd(&self) -> String {
        let mut line = String::new();
        self.print("=>");
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.replace("\r", ""); // some shells (windows) also add \r  thanks Alberto Segura
        line.truncate(line.len() - 1);
        line.to_lowercase()
    }

    pub fn cmd2(&self) -> String {
        let mut line = String::new();
        self.print("=>");
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.replace("\r", ""); // some shells (windows) also add \r  thanks Alberto Segura
        line.truncate(line.len() - 1);
        line
    }

    pub fn cmd_hex32(&self) -> Result<u32, ParseIntError> {
        let mut x = self.cmd();

        if x.ends_with('h') {
            x = x[0..x.len() - 1].to_string();
        }
        if x.starts_with("0x") {
            x = x[2..x.len()].to_string();
        }

        u32::from_str_radix(x.as_str(), 16)
    }

    pub fn cmd_hex64(&self) -> Result<u64, ParseIntError> {
        let mut x = self.cmd();
        if x.ends_with('h') {
            x = x[0..x.len() - 1].to_string();
        }
        if x.starts_with("0x") {
            x = x[2..x.len()].to_string();
        }

        u64::from_str_radix(x.as_str(), 16)
    }

    pub fn cmd_num(&self) -> Result<u64, ParseIntError> {
        self.cmd().as_str().parse::<u64>()
    }

    /*
    pub fn cmd_num<T>(&self) -> Result<T,ParseIntError> {
        self.cmd().as_str().parse::<T>()
    }*/

    pub fn help(&self) {
        log::trace!("--- help ---");
        log::trace!("q ...................... quit");
        log::trace!("cls .................... clear screen");
        log::trace!("h ...................... help");
        log::trace!("s ...................... stack");
        log::trace!("v ...................... vars");
        log::trace!("sv ..................... set verbose level 0, 1 or 2");
        log::trace!("r ...................... register show all");
        log::trace!("r reg .................. show reg");
        log::trace!("rc ..................... register change");
        log::trace!("f ...................... show all flags");
        log::trace!("fc ..................... clear all flags");
        log::trace!("fz ..................... toggle flag zero");
        log::trace!("fs ..................... toggle flag sign");
        log::trace!("c ...................... continue");
        log::trace!("b ...................... breakpoint list");
        log::trace!("ba ..................... breakpoint on address");
        log::trace!("bi ..................... breakpoint on instruction number");
        log::trace!("bmr .................... breakpoint on read memory");
        log::trace!("bmw .................... breakpoint on write memory");
        log::trace!("bmx .................... breakpoint on execute memory");
        log::trace!("bcmp ................... break on next cmp or test instruction");
        log::trace!("bc ..................... clear breakpoint");
        log::trace!("n ...................... next instruction");
        log::trace!("pc ..................... change pc (any arch)");
        log::trace!("eip .................... change eip");
        log::trace!("rip .................... change rip");
        log::trace!("push ................... push dword to the stack");
        log::trace!("pop .................... pop dword from stack");
        log::trace!("fpu .................... fpu view");
        log::trace!("md5 .................... check the md5 of a memory map");
        log::trace!("seh .................... view SEH");
        log::trace!("veh .................... view vectored execption pointer");
        log::trace!("m ...................... memory maps");
        log::trace!("ms ..................... memory filtered by keyword string");
        log::trace!("ma ..................... memory allocs");
        log::trace!("mc ..................... memory create map");
        log::trace!("mn ..................... memory name of an address");
        log::trace!("ml ..................... memory load file content to map");
        log::trace!("mr ..................... memory read, speficy ie: dword ptr [esi]");
        log::trace!(
            "mw ..................... memory write, speficy ie: dword ptr [esi]  and then: 1af"
        );
        log::trace!("mwb .................... memory write bytes, input spaced bytes");
        log::trace!("md ..................... memory dump");
        log::trace!("mrd .................... memory read dwords");
        log::trace!("mrq .................... memory read qwords");
        log::trace!("mds .................... memory dump string");
        log::trace!("mdw .................... memory dump wide string");
        log::trace!("mdd .................... memory dump to disk");
        log::trace!("mdda ................... memory dump all allocations to disk");
        log::trace!("mt ..................... memory test");
        log::trace!("r2 [addr] .............. spawn radare2 console if it's isntalled");
        log::trace!("ss ..................... search string in a specific map");
        log::trace!("sb ..................... search bytes in a specific map");
        log::trace!("sba .................... search bytes in all the maps");
        log::trace!("ssa .................... search string in all the maps");
        log::trace!("ll ..................... linked list walk");
        log::trace!("d ...................... dissasemble");
        log::trace!("dt ..................... dump structure");
        log::trace!("pos .................... print current position");
        log::trace!("enter .................. step into");
        log::trace!("tr ..................... trace reg");
        log::trace!("trc .................... trace regs clear");
        log::trace!("ldr .................... show ldr linked list");
        log::trace!("iat .................... find api name in all iat's ");
        log::trace!("iatx ................... addr to api name");
        log::trace!("iatd ................... dump the iat of specific module");
        log::trace!("dump ................... dump current state to disk");

        //log::trace!("o ...................... step over");
        log::trace!("");
        log::trace!("---");
    }

    pub fn spawn_radare2(addr: u64, emu: &mut Emu) {
        let mem = match emu.maps.get_mem_by_addr(addr) {
            Some(m) => m,
            None => {
                log::trace!("address not found on any map");
                return;
            }
        };

        let tmpfile = format!("/tmp/{}.r2", mem.get_name());
        mem.save_all(&tmpfile);

        let base = format!("0x{:x}", mem.get_base());
        let seek = format!("0x{:x}", addr);
        let arch_str;
        let bits;
        let precmd: String;
        if emu.cfg.arch.is_aarch64() {
            arch_str = "arm";
            bits = "64";
            let regs = emu.regs_aarch64();
            let mut cmd = String::new();
            for i in 0..31 {
                cmd.push_str(&format!("dr x{}={}; ", i, regs.x[i]));
            }
            cmd.push_str(&format!("dr sp={}; dr pc={};", regs.sp, regs.pc));
            precmd = cmd;
        } else if emu.cfg.is_x64() {
            arch_str = "x86";
            bits = "64";
            precmd = format!(
                "dr rax={}; dr rbx={}; dr rcx={}; dr rdx={}; dr rsi={};
                                  dr rdi={}; dr rbp={}; dr rsp={}; dr rip={}; dr r8={}
                                  dr r9={}; dr r10={}; dr r11={}; dr r12={}; dr r13={};
                                  dr r14={}; dr r15={}; decai -e model=qwen3-coder:30b; r2ai -e r2ai.model=qwen3-coder:30b;",
                                  emu.regs().rax, emu.regs().rbx, emu.regs().rcx, emu.regs().rdx,
                                  emu.regs().rsi, emu.regs().rdi, emu.regs().rbp, emu.regs().rsp,
                                  emu.regs().rip, emu.regs().r8, emu.regs().r9, emu.regs().r10,
                                  emu.regs().r11, emu.regs().r12, emu.regs().r13, emu.regs().r14,
                                  emu.regs().r15);
        } else {
            arch_str = "x86";
            bits = "32";
            precmd = format!(
                "dr eax={}; dr ebx={}; dr ecx={}; dr edx={}; dr esi={}; \
                dr edi={}; dr ebp={}; dr esp={}; dr eip={}; \
                decai -e model=qwen3-coder:30b; r2ai -e r2ai.model=qwen3-coder:30b;",
                emu.regs().get_eax(),
                emu.regs().get_ebx(),
                emu.regs().get_ecx(),
                emu.regs().get_edx(),
                emu.regs().get_esi(),
                emu.regs().get_edi(),
                emu.regs().get_ebp(),
                emu.regs().get_esp(),
                emu.regs().get_eip()
            );
        }
        let r2args = vec![
            "-n", "-a", arch_str, "-b", &bits, "-m", &base, "-s", &seek, "-c", &precmd, &tmpfile,
        ];

        log::trace!("spawning radare2 software.");

        match Command::new("radare2")
            .args(&r2args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
        {
            Ok(mut child) => {
                let _ = child.wait();
            }
            Err(e) => {
                log::error!("Install radare first! {}", e);
                return;
            }
        }

        if let Err(e) = fs::remove_file(&tmpfile) {
            if e.kind() != io::ErrorKind::NotFound {
                log::error!("temporal file not found");
            }
        }
    }

    pub fn spawn_console(emu: &mut Emu) {
        if !emu.cfg.console_enabled {
            return;
        }

        let con = Console::new();
        if emu.pos > 0 {
            emu.pos -= 1;
        }
        loop {
            let cmd: String;

            if let Some(pcmd) = &emu.cfg.command {
                cmd = pcmd.to_string();
            } else {
                cmd = con.cmd();
            }

            match cmd.as_str() {
                "q" => std::process::exit(1),
                "h" => con.help(),
                "r" => {
                    if emu.cfg.arch.is_aarch64() {
                        emu.featured_regs_aarch64();
                    } else if emu.cfg.is_x64() {
                        emu.featured_regs64();
                    } else {
                        emu.featured_regs32();
                    }
                }
                "r rax" => emu.regs().show_rax(&emu.maps, 0),
                "r rbx" => emu.regs().show_rbx(&emu.maps, 0),
                "r rcx" => emu.regs().show_rcx(&emu.maps, 0),
                "r rdx" => emu.regs().show_rdx(&emu.maps, 0),
                "r rsi" => emu.regs().show_rsi(&emu.maps, 0),
                "r rdi" => emu.regs().show_rdi(&emu.maps, 0),
                "r rbp" => log::trace!("\trbp: 0x{:x}", emu.regs().rbp),
                "r rsp" => log::trace!("\trsp: 0x{:x}", emu.regs().rsp),
                "r rip" => log::trace!("\trip: 0x{:x}", emu.regs().rip),
                "r eax" => emu.regs().show_eax(&emu.maps, 0),
                "r ebx" => emu.regs().show_ebx(&emu.maps, 0),
                "r ecx" => emu.regs().show_ecx(&emu.maps, 0),
                "r edx" => emu.regs().show_edx(&emu.maps, 0),
                "r esi" => emu.regs().show_esi(&emu.maps, 0),
                "r edi" => emu.regs().show_edi(&emu.maps, 0),
                "r esp" => log::trace!("\tesp: 0x{:x}", emu.regs().get_esp() as u32),
                "r ebp" => log::trace!("\tebp: 0x{:x}", emu.regs().get_ebp() as u32),
                "r eip" => log::trace!("\teip: 0x{:x}", emu.regs().get_eip() as u32),
                "r r8" => emu.regs().show_r8(&emu.maps, 0),
                "r r9" => emu.regs().show_r9(&emu.maps, 0),
                "r r10" => emu.regs().show_r10(&emu.maps, 0),
                "r r11" => emu.regs().show_r11(&emu.maps, 0),
                "r r12" => emu.regs().show_r12(&emu.maps, 0),
                "r r13" => emu.regs().show_r13(&emu.maps, 0),
                "r r14" => emu.regs().show_r14(&emu.maps, 0),
                "r r15" => emu.regs().show_r15(&emu.maps, 0),
                "r r8d" => emu.regs().show_r8d(&emu.maps, 0),
                "r r9d" => emu.regs().show_r9d(&emu.maps, 0),
                "r r10d" => emu.regs().show_r10d(&emu.maps, 0),
                "r r11d" => emu.regs().show_r11d(&emu.maps, 0),
                "r r12d" => emu.regs().show_r12d(&emu.maps, 0),
                "r r13d" => emu.regs().show_r13d(&emu.maps, 0),
                "r r14d" => emu.regs().show_r14d(&emu.maps, 0),
                "r r15d" => emu.regs().show_r15d(&emu.maps, 0),
                "r r8w" => emu.regs().show_r8w(&emu.maps, 0),
                "r r9w" => emu.regs().show_r9w(&emu.maps, 0),
                "r r10w" => emu.regs().show_r10w(&emu.maps, 0),
                "r r11w" => emu.regs().show_r11w(&emu.maps, 0),
                "r r12w" => emu.regs().show_r12w(&emu.maps, 0),
                "r r13w" => emu.regs().show_r13w(&emu.maps, 0),
                "r r14w" => emu.regs().show_r14w(&emu.maps, 0),
                "r r15w" => emu.regs().show_r15w(&emu.maps, 0),
                "r r8l" => emu.regs().show_r8l(&emu.maps, 0),
                "r r9l" => emu.regs().show_r9l(&emu.maps, 0),
                "r r10l" => emu.regs().show_r10l(&emu.maps, 0),
                "r r11l" => emu.regs().show_r11l(&emu.maps, 0),
                "r r12l" => emu.regs().show_r12l(&emu.maps, 0),
                "r r13l" => emu.regs().show_r13l(&emu.maps, 0),
                "r r14l" => emu.regs().show_r14l(&emu.maps, 0),
                "r r15l" => emu.regs().show_r15l(&emu.maps, 0),
                "r xmm0" => log::trace!("\txmm0: 0x{:x}", emu.regs().xmm0),
                "r xmm1" => log::trace!("\txmm1: 0x{:x}", emu.regs().xmm1),
                "r xmm2" => log::trace!("\txmm2: 0x{:x}", emu.regs().xmm2),
                "r xmm3" => log::trace!("\txmm3: 0x{:x}", emu.regs().xmm3),
                "r xmm4" => log::trace!("\txmm4: 0x{:x}", emu.regs().xmm4),
                "r xmm5" => log::trace!("\txmm5: 0x{:x}", emu.regs().xmm5),
                "r xmm6" => log::trace!("\txmm6: 0x{:x}", emu.regs().xmm6),
                "r xmm7" => log::trace!("\txmm7: 0x{:x}", emu.regs().xmm7),
                "r xmm8" => log::trace!("\txmm8: 0x{:x}", emu.regs().xmm8),
                "r xmm9" => log::trace!("\txmm9: 0x{:x}", emu.regs().xmm9),
                "r xmm10" => log::trace!("\txmm10: 0x{:x}", emu.regs().xmm10),
                "r xmm11" => log::trace!("\txmm11: 0x{:x}", emu.regs().xmm11),
                "r xmm12" => log::trace!("\txmm12: 0x{:x}", emu.regs().xmm12),
                "r xmm13" => log::trace!("\txmm13: 0x{:x}", emu.regs().xmm13),
                "r xmm14" => log::trace!("\txmm14: 0x{:x}", emu.regs().xmm14),
                "r xmm15" => log::trace!("\txmm15: 0x{:x}", emu.regs().xmm15),
                "r ymm0" => log::trace!("\tymm0: 0x{:x}", emu.regs().ymm0),
                "r ymm1" => log::trace!("\tymm1: 0x{:x}", emu.regs().ymm1),
                "r ymm2" => log::trace!("\tymm2: 0x{:x}", emu.regs().ymm2),
                "r ymm3" => log::trace!("\tymm3: 0x{:x}", emu.regs().ymm3),
                "r ymm4" => log::trace!("\tymm4: 0x{:x}", emu.regs().ymm4),
                "r ymm5" => log::trace!("\tymm5: 0x{:x}", emu.regs().ymm5),
                "r ymm6" => log::trace!("\tymm6: 0x{:x}", emu.regs().ymm6),
                "r ymm7" => log::trace!("\tymm7: 0x{:x}", emu.regs().ymm7),
                "r ymm8" => log::trace!("\tymm8: 0x{:x}", emu.regs().ymm8),
                "r ymm9" => log::trace!("\tymm9: 0x{:x}", emu.regs().ymm9),
                "r ymm10" => log::trace!("\tymm10: 0x{:x}", emu.regs().ymm10),
                "r ymm11" => log::trace!("\tymm11: 0x{:x}", emu.regs().ymm11),
                "r ymm12" => log::trace!("\tymm12: 0x{:x}", emu.regs().ymm12),
                "r ymm13" => log::trace!("\tymm13: 0x{:x}", emu.regs().ymm13),
                "r ymm14" => log::trace!("\tymm14: 0x{:x}", emu.regs().ymm14),
                "r ymm15" => log::trace!("\tymm15: 0x{:x}", emu.regs().ymm15),

                "rc" => {
                    con.print("register name");
                    let reg = con.cmd();
                    con.print("value");
                    let value = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value");
                            continue;
                        }
                    };
                    if emu.cfg.arch.is_aarch64() {
                        if !emu.regs_aarch64_mut().set_by_name(reg.as_str(), value) {
                            log::trace!("unknown aarch64 register: {}", reg);
                        }
                    } else {
                        emu.regs_mut().set_by_name(reg.as_str(), value);
                    }
                }
                "mr" | "rm" => {
                    con.print("memory argument");
                    let operand = con.cmd();
                    let addr: u64 = emu.memory_operand_to_address(operand.as_str());
                    let value = match emu.memory_read(operand.as_str()) {
                        Some(v) => v,
                        None => {
                            log::trace!("bad address.");
                            continue;
                        }
                    };
                    log::trace!("0x{:x}: 0x{:x}", to32!(addr), value);
                }
                "mw" | "wm" => {
                    con.print("memory argument");
                    let operand = con.cmd();
                    con.print("value");
                    let value = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value.");
                            continue;
                        }
                    };
                    if emu.memory_write(operand.as_str(), value) {
                        log::trace!("done.");
                    } else {
                        log::trace!("cannot write there.");
                    }
                }
                "mwb" => {
                    con.print("addr");
                    let addr = match con.cmd_hex64() {
                        Ok(a) => a,
                        Err(_) => {
                            log::trace!("bad hex value");
                            continue;
                        }
                    };
                    con.print("spaced bytes");
                    let bytes = con.cmd();
                    emu.maps.write_spaced_bytes(addr, &bytes);
                    log::trace!("done.");
                }
                "b" => {
                    emu.bp.show();
                }
                "ba" => {
                    con.print("address");
                    let addr = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value.");
                            continue;
                        }
                    };
                    emu.bp.add_bp(addr);
                }
                "bmr" => {
                    con.print("address");
                    let addr = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value.");
                            continue;
                        }
                    };
                    emu.bp.add_bp_mem_read(addr);
                }
                "bmw" => {
                    con.print("address");
                    let addr = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value.");
                            continue;
                        }
                    };
                    emu.bp.add_bp_mem_write(addr);
                }
                "bi" => {
                    con.print("instruction number");
                    let num = match con.cmd_num() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad instruction number.");
                            continue;
                        }
                    };
                    emu.bp.add_bp_instruction(num);
                    emu.exp = num;
                }
                "bc" => {
                    emu.bp.clear_bp();
                    emu.exp = emu.pos + 1;
                }
                "bcmp" => {
                    emu.break_on_next_cmp = true;
                }
                "cls" => log::trace!("{}", emu.colors.clear_screen),
                "s" => {
                    if emu.cfg.arch.is_aarch64() {
                        emu.maps.dump_qwords(emu.sp(), 10);
                    } else if emu.cfg.is_x64() {
                        emu.maps.dump_qwords(emu.regs().rsp, 10);
                    } else {
                        emu.maps.dump_dwords(emu.regs().get_esp(), 10);
                    }
                }
                "v" => {
                    if emu.cfg.arch.is_aarch64() {
                        // aarch64: dump around SP (no frame pointer convention guaranteed)
                        emu.maps.dump_qwords(emu.sp().wrapping_sub(0x100), 100);
                    } else if emu.cfg.is_x64() {
                        emu.maps.dump_qwords(emu.regs().rbp - 0x100, 100);
                    } else {
                        emu.maps.dump_dwords(emu.regs().get_ebp() - 0x100, 100);
                        emu.maps.get_mem("stack").print_dwords_from_to(
                            emu.regs().get_ebp(),
                            emu.regs().get_ebp() + 0x100,
                        );
                    }
                }
                "sv" => {
                    con.print("verbose level");
                    emu.cfg.verbose = match con.cmd_num() {
                        Ok(v) => to32!(v),
                        Err(_) => {
                            log::trace!("incorrect verbose level, set 0, 1 or 2");
                            continue;
                        }
                    };
                }
                "tr" => {
                    con.print("register");
                    let reg = con.cmd();
                    emu.cfg.trace_reg = true;
                    emu.cfg.reg_names.push(reg);
                }
                "trc" => {
                    emu.cfg.trace_reg = false;
                    emu.cfg.reg_names.clear();
                }
                "pos" => {
                    log::trace!("pos = 0x{:x}", emu.pos);
                }
                "c" => {
                    emu.exp += 1;
                    emu.is_running.store(1, atomic::Ordering::Relaxed);
                    return;
                }
                "cr" => {
                    emu.break_on_next_return = true;
                    emu.is_running.store(1, atomic::Ordering::Relaxed);
                    return;
                }
                "f" => {
                    if emu.cfg.arch.is_aarch64() {
                        let nzcv = &emu.regs_aarch64().nzcv;
                        log::trace!("NZCV: N={} Z={} C={} V={}", nzcv.n, nzcv.z, nzcv.c, nzcv.v);
                    } else {
                        emu.flags().print();
                    }
                }
                "fc" => {
                    if emu.cfg.arch.is_aarch64() {
                        let nzcv = &mut emu.regs_aarch64_mut().nzcv;
                        nzcv.n = false;
                        nzcv.z = false;
                        nzcv.c = false;
                        nzcv.v = false;
                    } else {
                        emu.flags_mut().clear();
                    }
                }
                "fz" => {
                    if emu.cfg.arch.is_aarch64() {
                        let nzcv = &mut emu.regs_aarch64_mut().nzcv;
                        nzcv.z = !nzcv.z;
                    } else {
                        emu.flags_mut().f_zf = !emu.flag_zf();
                    }
                }
                "fs" => {
                    if emu.cfg.arch.is_aarch64() {
                        let nzcv = &mut emu.regs_aarch64_mut().nzcv;
                        nzcv.n = !nzcv.n;
                    } else {
                        emu.flags_mut().f_sf = !emu.flag_sf();
                    }
                }
                "mc" => {
                    con.print("name ");
                    let name = con.cmd();
                    con.print("size ");
                    let sz = match con.cmd_num() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad size.");
                            continue;
                        }
                    };

                    let addr = match emu.maps.alloc(sz) {
                        Some(a) => a,
                        None => {
                            log::trace!("memory full");
                            continue;
                        }
                    };
                    emu.maps
                        .create_map(&name, addr, sz, Permission::READ_WRITE_EXECUTE)
                        .expect("cannot create map from console mc");
                    log::trace!("allocated {} at 0x{:x} sz: {}", name, addr, sz);
                }
                "mca" => {
                    con.print("name ");
                    let name = con.cmd();
                    con.print("address ");
                    let addr = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad size.");
                            continue;
                        }
                    };

                    con.print("size ");
                    let sz = match con.cmd_num() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad size.");
                            continue;
                        }
                    };

                    emu.maps
                        .create_map(&name, addr, sz, Permission::READ_WRITE_EXECUTE)
                        .expect("cannot create map from console mca");
                    log::trace!("allocated {} at 0x{:x} sz: {}", name, addr, sz);
                }
                "ml" => {
                    con.print("map name");
                    let name = con.cmd();
                    con.print("filename");
                    let filename = con.cmd();
                    emu.maps.get_mem_mut(name.as_str()).load(filename.as_str());
                }
                "mn" => {
                    con.print("address");
                    let addr = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value.");
                            continue;
                        }
                    };
                    let name = match emu.maps.get_addr_name(addr) {
                        Some(n) => n,
                        None => {
                            if !emu.cfg.skip_unimplemented {
                                log::trace!("address not found on any map");
                                continue;
                            }

                            "code"
                        }
                    };

                    let mem = emu
                        .maps
                        .get_mem_by_addr(addr)
                        .expect("address not found on any map");
                    if emu.cfg.is_x64() {
                        log::trace!(
                            "map: {} 0x{:x}-0x{:x} ({})",
                            name,
                            mem.get_base(),
                            mem.get_bottom(),
                            mem.size()
                        );
                    } else {
                        log::trace!(
                            "map: {} 0x{:x}-0x{:x} ({})",
                            name,
                            to32!(mem.get_base()),
                            to32!(mem.get_bottom()),
                            mem.size()
                        );
                    }
                }
                "ma" => {
                    emu.maps.show_allocs();
                }
                "md" => {
                    con.print("address");
                    let addr = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value.");
                            continue;
                        }
                    };
                    emu.maps.dump(addr);
                }
                "mrd" => {
                    con.print("address");
                    let addr = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value.");
                            continue;
                        }
                    };
                    emu.maps.dump_dwords(addr, 10);
                }
                "mrq" => {
                    con.print("address");
                    let addr = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value.");
                            continue;
                        }
                    };
                    emu.maps.dump_qwords(addr, 10);
                }
                "mds" => {
                    con.print("address");
                    let addr = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value.");
                            continue;
                        }
                    };
                    if emu.cfg.is_x64() {
                        log::trace!("0x{:x}: '{}'", addr, emu.maps.read_string(addr));
                    } else {
                        log::trace!("0x{:x}: '{}'", to32!(addr), emu.maps.read_string(addr));
                    }
                }
                "mdw" => {
                    con.print("address");
                    let addr = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value.");
                            continue;
                        }
                    };
                    if emu.cfg.is_x64() {
                        log::trace!("0x{:x}: '{}'", addr, emu.maps.read_wide_string(addr));
                    } else {
                        log::trace!("0x{:x}: '{}'", to32!(addr), emu.maps.read_wide_string(addr));
                    }
                }
                "mdd" => {
                    con.print("address");
                    let addr = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value.");
                            continue;
                        }
                    };
                    con.print("size");
                    let sz = match con.cmd_num() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad numeric decimal value.");
                            continue;
                        }
                    };
                    if sz > 0 {
                        con.print("file");
                        let filename = con.cmd();
                        emu.maps.save(addr, sz, filename);
                    }
                }
                "mdda" => {
                    con.print("path:");
                    let path = con.cmd2();
                    emu.maps.save_all_allocs(path);
                }
                "mt" => {
                    if emu.maps.mem_test() {
                        log::trace!("mem test passed ok.");
                    } else {
                        log::trace!("memory errors.");
                    }
                }
                "pc" => {
                    con.print("=");
                    let addr = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value");
                            continue;
                        }
                    };
                    emu.set_pc(addr);
                    emu.force_break = true;
                    break;
                }
                "eip" => {
                    con.print("=");
                    let addr = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value");
                            continue;
                        }
                    };
                    emu.set_eip(addr, false);
                    emu.force_break = true;
                    break;
                }
                "rip" => {
                    con.print("=");
                    let addr = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value");
                            continue;
                        }
                    };
                    emu.set_rip(addr, false);
                    emu.force_break = true;
                    break;
                    //emu.regs_mut().rip = addr;
                }
                "push" => {
                    con.print("value");
                    if emu.cfg.arch.is_aarch64() || emu.cfg.is_x64() {
                        let value = match con.cmd_hex64() {
                            Ok(v) => v,
                            Err(_) => {
                                log::trace!("bad hex value");
                                continue;
                            }
                        };
                        emu.stack_push64(value);
                    } else {
                        let value = match con.cmd_hex32() {
                            Ok(v) => v,
                            Err(_) => {
                                log::trace!("bad hex value");
                                continue;
                            }
                        };
                        emu.stack_push32(value);
                    }
                    log::trace!("pushed.");
                }
                "pop" => {
                    if emu.cfg.arch.is_aarch64() || emu.cfg.is_x64() {
                        let value = emu.stack_pop64(false).unwrap_or(0);
                        log::trace!("poped value 0x{:x}", value);
                    } else {
                        let value = emu.stack_pop32(false).unwrap_or(0);
                        log::trace!("poped value 0x{:x}", value);
                    }
                }
                "fpu" => {
                    emu.fpu_mut().print();
                }
                "md5" => {
                    con.print("map name");
                    let mem_name = con.cmd();
                    let mem = emu.maps.get_mem(&mem_name);
                    let md5 = mem.md5();
                    log::trace!("md5sum: {:x}", md5);
                }
                "ss" => {
                    con.print("map name");
                    let mem_name = con.cmd();
                    con.print("string");
                    let kw = con.cmd2();
                    let result = match emu.maps.search_string(&kw, &mem_name) {
                        Some(v) => v,
                        None => {
                            log::trace!("not found.");
                            continue;
                        }
                    };
                    for addr in result.iter() {
                        if emu.cfg.is_x64() {
                            log::trace!("found 0x{:x} '{}'", *addr, emu.maps.read_string(*addr));
                        } else {
                            log::trace!(
                                "found 0x{:x} '{}'",
                                *addr as u32,
                                emu.maps.read_string(*addr)
                            );
                        }
                    }
                }
                "sb" => {
                    con.print("map name");
                    let mem_name = con.cmd();
                    con.print("spaced bytes");
                    let sbs = con.cmd();
                    let results = emu.maps.search_spaced_bytes(&sbs, &mem_name);
                    if results.is_empty() {
                        log::trace!("not found.");
                    } else if emu.cfg.is_x64() {
                        for addr in results.iter() {
                            log::trace!("found at 0x{:x}", addr);
                        }
                    } else {
                        for addr in results.iter() {
                            log::trace!("found at 0x{:x}", to32!(addr));
                        }
                    }
                }
                "sba" => {
                    con.print("spaced bytes");
                    let sbs = con.cmd();
                    let results = emu.maps.search_spaced_bytes_in_all(&sbs);
                    if results.is_empty() {
                        log::trace!("not found.");
                    } else if emu.cfg.is_x64() {
                        for addr in results.iter() {
                            log::trace!("found at 0x{:x}", addr);
                        }
                    } else {
                        for addr in results.iter() {
                            log::trace!("found at 0x{:x}", to32!(addr));
                        }
                    }
                }
                "ssa" => {
                    con.print("string");
                    let kw = con.cmd2();
                    emu.maps.search_string_in_all(kw);
                }
                "seh" => {
                    log::trace!("0x{:x}", emu.seh());
                }
                "veh" => {
                    log::trace!("0x{:x}", emu.veh());
                }
                "ll" => {
                    con.print("ptr");
                    let ptr1 = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value");
                            continue;
                        }
                    };
                    let mut ptr = ptr1;
                    loop {
                        log::trace!("- 0x{:x}", ptr);
                        ptr = match emu.maps.read_dword(ptr) {
                            Some(v) => v.into(),
                            None => break,
                        };
                        if ptr == 0 || ptr == ptr1 {
                            break;
                        }
                    }
                }
                "n" | "" => {
                    //emu.exp = emu.pos + 1;
                    let prev_verbose = emu.cfg.verbose;
                    emu.cfg.verbose = 3;
                    emu.step();
                    emu.cfg.verbose = prev_verbose;
                    //return;
                }
                "m" => emu.maps.print_maps(),
                "ms" => {
                    con.print("keyword");
                    let kw = con.cmd2();
                    emu.maps.print_maps_keyword(&kw);
                }
                "d" => {
                    con.print("address");
                    let addr = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value");
                            continue;
                        }
                    };
                    log::trace!("{}", emu.disassemble(addr, 10));
                }
                "ldr" => {
                    if emu.cfg.is_x64() {
                        peb64::show_linked_modules(emu);
                    } else {
                        peb32::show_linked_modules(emu);
                    }
                }
                "iat" => {
                    con.print("api keyword");
                    let kw = con.cmd2();
                    let addr: u64;
                    let lib: String;
                    let name: String;

                    if emu.cfg.is_x64() {
                        (addr, lib, name) = winapi64::kernel32::search_api_name(emu, &kw);
                    } else {
                        (addr, lib, name) = winapi32::kernel32::search_api_name(emu, &kw);
                    }

                    if addr == 0 {
                        log::trace!("api not found");
                    } else {
                        log::trace!("found: 0x{:x} {}!{}", addr, lib, name);
                    }
                }
                "iatx" => {
                    // addr to name
                    con.print("api addr");
                    let addr = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value.");
                            continue;
                        }
                    };

                    let name: String = if emu.cfg.is_x64() {
                        winapi64::kernel32::resolve_api_addr_to_name(emu, addr)
                    } else {
                        winapi32::kernel32::resolve_api_addr_to_name(emu, addr)
                    };

                    if name.is_empty() {
                        log::trace!("api addr not found");
                    } else {
                        log::trace!("found: 0x{:x} {}", addr, name);
                    }
                }
                "iatd" => {
                    con.print("module");
                    let lib = con.cmd2().to_lowercase();
                    if emu.cfg.is_x64() {
                        winapi64::kernel32::dump_module_iat(emu, &lib);
                    } else {
                        winapi32::kernel32::dump_module_iat(emu, &lib);
                    }
                }
                "dump" => {
                    if emu.cfg.dump_filename.is_some() {
                        serialization::Serialization::dump(
                            emu,
                            emu.cfg.dump_filename.as_ref().unwrap(),
                        );
                    }
                }
                "dt" => {
                    con.print("structure");
                    let struc = con.cmd();
                    con.print("address");
                    let addr = match con.cmd_hex64() {
                        Ok(v) => v,
                        Err(_) => {
                            log::trace!("bad hex value");
                            continue;
                        }
                    };

                    match struc.as_str() {
                        "peb" => {
                            let s = structures::PEB::load(addr, &emu.maps);
                            s.print();
                        }
                        "teb" => {
                            let s = structures::TEB::load(addr, &emu.maps);
                            s.print();
                        }
                        "peb_ldr_data" => {
                            let s = structures::PebLdrData::load(addr, &emu.maps);
                            s.print();
                        }
                        "ldr_data_table_entry" => {
                            let s = structures::LdrDataTableEntry::load(addr, &emu.maps);
                            s.print();
                        }
                        "list_entry" => {
                            let s = structures::ListEntry::load(addr, &emu.maps);
                            s.print();
                        }
                        "peb64" => {
                            let s = structures::PEB64::load(addr, &emu.maps);
                            s.print();
                        }
                        "teb64" => {
                            let s = structures::TEB64::load(addr, &emu.maps);
                            s.print();
                        }
                        "peb_ldr_data64" => {
                            let s = structures::PebLdrData64::load(addr, &emu.maps);
                            s.print();
                        }
                        "ldr_data_table_entry64" => {
                            let s = structures::LdrDataTableEntry64::load(addr, &emu.maps);
                            s.print();
                        }
                        "list_entry64" => {
                            let s = structures::ListEntry64::load(addr, &emu.maps);
                            s.print();
                        }
                        "cppeh_record" => {
                            let s = structures::CppEhRecord::load(addr, &emu.maps);
                            s.print();
                        }
                        "exception_pointers" => {
                            let s = structures::ExceptionPointers::load(addr, &emu.maps);
                            s.print();
                        }
                        "eh3_exception_registgration" => {
                            let s = structures::Eh3ExceptionRegistration::load(addr, &emu.maps);
                            s.print();
                        }
                        "memory_basic_information" => {
                            let s = structures::MemoryBasicInformation::load(addr, &emu.maps);
                            s.print();
                        }
                        "image_export_directory" => {
                            let s = structures::ImageExportDirectory::load(addr, &emu.maps);
                            s.print();
                        }

                        _ => log::trace!("unrecognized structure."),
                    }
                } // end dt command

                _ => {
                    // Handle "r <regname>" for aarch64 registers dynamically
                    if cmd.starts_with("r ") && emu.cfg.arch.is_aarch64() {
                        let reg_name = &cmd[2..];
                        match emu.regs_aarch64().get_by_name(reg_name) {
                            Some(val) => log::trace!("\t{}: 0x{:016x}", reg_name, val),
                            None => log::trace!("unknown aarch64 register: {}", reg_name),
                        }
                    } else if cmd.starts_with("m ") {
                        let parts: Vec<&str> = cmd.split_whitespace().collect();
                        if parts.len() >= 2 {
                            emu.maps.print_maps_keyword(parts[1]);
                        }
                    } else if cmd.starts_with("r2 ") {
                        let parts: Vec<&str> = cmd.split_whitespace().collect();
                        if parts.len() >= 2 {
                            if let Ok(addr) =
                                u64::from_str_radix(parts[1].trim_start_matches("0x"), 16)
                            {
                                Console::spawn_radare2(addr, emu);
                            } else {
                                println!("wrong hexa parameter");
                            }
                        } else {
                            log::trace!("r2 command needs 1 parameter, the an hex address");
                        }
                    } else {
                        log::trace!("command not found, type h");
                    }
                }
            } // match commands

            if emu.cfg.command.is_some() {
                std::process::exit(1);
            }
        } // end loop
    } // end commands function
}

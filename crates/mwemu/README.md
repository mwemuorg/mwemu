
# mwemu commandline tool

## Main flags

- verbose flag:
    No flag            It will show only API calls.
    -v                  It will show only API calls and some messages like polymorfic code, etc.    
    -vv                 Show assembly code, but display rep instruccions only once.
    -vvv                Show assembly code and show every rep.

- target file
    -f [filepath]

- maps folder
    By default it looks for the maps in ./maps/windows/x86/ or ./maps/windows/x86_64/
    -m [path]

- toggle 64bits 
    No flags        By default will use x86 32bits emulation.
    -6              Will use 64bits emulation. Will be rejected with
                    a clear error if the input file is a PE32/x86 binary;
                    omit -6 for PE32 inputs.

- spawn console
    -c [number]     Spawn the console after emulating n instructions.

## Examples

- basic usage:

```
32bits:
cargo run --release -- -f test/sc32win_donut.bin -vv

64bits:
cargo run --release -- -f test/sc64win_metasploit.bin -vv -6

console:
cargo run --release -- -f test/sc64win_metasploit.bin -vv -6 -c 100

memroy trace:
cargo run --release -- -f test/sc32win_donut.bin -vvv -c 121 --memory

register trace:
cargo run --release -- -f test/sc32win_donut.bin -vvv -c 121 --reg eax,ebx

stack trace:
cargo run --release -- -f test/sc32win_donut.bin --stack_trace

inspect memory: (experimental)
cargo run --release -- -f test/sc32win_donut.bin -i 'dword ptr [ebp + 0x24]'

```


## --help

```
MWEMU emulator for malware
@sha0coder

USAGE:
    mwemu [FLAGS] [OPTIONS]

FLAGS:
    -6, --64bits         enable 64bits architecture emulation
        --banzai         skip unimplemented instructions, and keep up emulating what can be emulated
    -F, --fpu            trace the fpu states.
    -h, --handle         handle Ctrl+C to spawn console
        --help           Prints help information
    -l, --loops          show loop interations, it is slow.
    -m, --memory         trace all the memory accesses read and write.
    -n, --nocolors       print without colors for redirectin to a file >out
    -r, --regs           print the register values in every step.
    -p, --stack_trace    trace stack on push/pop
    -t, --test           test mode
    -V, --version        Prints version information
    -v, --verbose        -vv for view the assembly, -v only messages, without verbose only see the api calls and goes
                         faster

OPTIONS:
    -b, --base <ADDRESS>               set base address for code
    -c, --console <NUMBER>             select in which moment will spawn the console to inspect.
    -C, --console_addr <ADDRESS>       spawn console on first eip = address
    -d, --dump <FILE>                  load from dump.
    -a, --entry <ADDRESS>              entry point of the shellcode, by default starts from the beginning.
    -e, --exit <POSITION>              exit position of the shellcode
    -f, --filename <FILE>              set the shellcode binary file.
    -i, --inspect <DIRECTION>          monitor memory like: -i 'dword ptr [ebp + 0x24]
    -M, --maps <PATH>                  select the memory maps folder
        --mxcsr <MXCSR>                set mxcsr register
        --r10 <R10>                    set r10 register
        --r11 <R11>                    set r11 register
        --r12 <R12>                    set r12 register
        --r13 <R13>                    set r13 register
        --r14 <R14>                    set r14 register
        --r15 <R15>                    set r15 register
        --r8 <R8>                      set r8 register
        --r9 <R9>                      set r9 register
        --rax <RAX>                    set rax register
        --rbp <RBP>                    set rbp register
        --rbx <RBX>                    set rbx register
        --rcx <RCX>                    set rcx register
        --rdi <RDI>                    set rdi register
        --rdx <RDX>                    set rdx register
    -R, --reg <REGISTER1,REGISTER2>    trace a specific register in every step, value and content
        --rflags <RFLAGS>              set rflags register
        --rsi <RSI>                    set rsi register
        --rsp <RSP>                    set rsp register
    -x, --script <SCRIPT>              launch an emulation script, see scripts_examples folder
        --stack_address <ADDRESS>      set stack address
    -s, --string <ADDRESS>             monitor string on a specific address
    -T, --trace <TRACE_FILENAME>       output trace to specified file
    -S, --trace_start <TRACE_START>    start trace at specified position
```

# Other options

For having more control of the binary use:

- rust library libmwemu
- python library pymwemu



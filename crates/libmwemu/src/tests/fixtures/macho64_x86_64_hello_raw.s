.text
.globl _start
.p2align 4, 0x90
_start:
    movq $0x2000004, %rax
    movq $1, %rdi
    leaq _msg(%rip), %rsi
    movq $14, %rdx
    syscall

    movq $0x2000001, %rax
    xorl %edi, %edi
    syscall

.section __TEXT,__cstring,cstring_literals
_msg:
    .asciz "Hello, World!\n"

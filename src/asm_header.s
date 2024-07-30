BITS 64

section .bss
    reg: resb 16
    mem: resq 1
    ret_addr: resq 1
    mem_read_callback: resq 1
    mem_write_callback: resq 1

section .text
global _main
export _main
global _DllMain

_DllMain:
    mov eax, 1
    ret

_main:
    mov [ret_addr], rsp
    mov [mem], rcx
    mov r8, rcx
BITS 64

section .bss
    ret_addr: resq 1
    instruction_count: resq 1
    mem_read_callback: resq 1
    mem_write_callback: resq 1

section .text
global _main
export _main
global _DllMain

_DllMain:
    mov eax, 1
    ret

_set_flags:
    jc _set_carry
    jmp _clear_carry
_sf1:
    jz __set_zero
    jmp __clear_zero
_sf2:
    ret

_set_carry:
    mov r14, 1
    jmp _sf1

_clear_carry:
    mov r14, 0
    jmp _sf1

__set_zero:
    mov r15, 1
    jmp _sf2

__clear_zero:
    mov r15, 0
    jmp _sf2

_main:
    mov [ret_addr], rsp
    mov r12, rcx
    mov r13, rdx
    mov [mem_read_callback], r8
    mov [mem_write_callback], r9
    mov rax, [rsp + 40]
    mov [instruction_count], rax
    sub rsp, 8
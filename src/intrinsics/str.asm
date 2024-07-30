    ; str
    mov cl, [reg + {a}]
    add cl, {o}
    movzx rdx, cl
    mov bl, [reg + {b}]
    ; mov rcx, [mem]
    ; call [mem_write_callback]
    mov [r8 + rdx], bl
    ; mov r8, [mem]
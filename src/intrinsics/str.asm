    ; str
    mov cl, [reg + {a}]
    add cl, {o}
    movzx rdx, cl
    push rdx
    mov rcx, [mem]
    call [mem_write_callback]
    mov r8, [mem]
    pop rdx
    mov bl, [reg + {b}]
    mov [r8 + rdx], bl
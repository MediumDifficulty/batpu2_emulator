    ; str
    mov cl, [reg + {a}]
    add cl, {o}
    movzx rdx, cl
    mov r8, [mem]
    mov bl, [reg + {b}]
    mov [r8 + rdx], bl    
    mov rcx, [mem]
    call [mem_write_callback]
    mov r8, [mem]

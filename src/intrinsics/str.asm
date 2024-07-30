    ; str
    mov cl, [r13 + {a}]
    add cl, {o}
    movzx rdx, cl
    mov bl, [r13 + {b}]
    mov [r12 + rdx], bl    
    mov rcx, r12
    call [mem_write_callback]

    ; str
    mov cl, [reg + {a}]
    add cl, {o}
    movzx rdx, cl
    mov bl, [reg + {b}]
    mov [r12 + rdx], bl    
    mov rcx, r12
    call [mem_write_callback]

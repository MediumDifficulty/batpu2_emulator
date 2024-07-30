    ; lod
    mov cl, [reg + {a}]
    add cl, {o}
    movzx rdx, cl
    push rdx
    mov rcx, [mem]
    call [mem_read_callback]
    mov r8, [mem]
    pop rdx
    mov cl, [r8 + rdx]
    mov {dest}, cl
    ; lod
    mov cl, [reg + {a}]
    add cl, {o}
    movzx rdx, cl
    sub rsp, 8
    push rdx
    mov rcx, [mem]
    call [mem_read_callback]
    mov r8, [mem]
    pop rdx
    add rsp, 8
    mov cl, [r8 + rdx]
    mov {dest}, cl
    ; lod
    mov cl, [reg + {a}]
    add cl, {o}
    movzx rdx, cl
    sub rsp, 8
    push rdx
    mov rcx, r12
    call [mem_read_callback]
    pop rdx
    add rsp, 8
    mov cl, [r12 + rdx]
    mov {dest}, cl
    mov cl, [reg + {a}]
    add cl, {o}
    movzx rcx, cl
    mov dl, [r8 + rcx]
    mov {dest}, dl
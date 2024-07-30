    ; xor
    mov cl, [r13 + {a}]
    mov dl, [r13 + {b}]
    mov {dest}, cl
    xor {dest}, dl
    call _set_flags
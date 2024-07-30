    mov cl, [reg + {a}]
    mov dl, [reg + {b}]
    mov {dest}, cl
    xor {dest}, dl
    call _set_flags
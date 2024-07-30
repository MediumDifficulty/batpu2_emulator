    mov cl, [reg + {a}]
    mov dl, [reg + {b}]
    mov {dest}, cl
    add {dest}, dl
    call _set_flags
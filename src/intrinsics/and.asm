    mov cl, [reg + {a}]
    mov dl, [reg + {b}]
    mov {dest}, cl
    and {dest}, dl
    call _set_flags
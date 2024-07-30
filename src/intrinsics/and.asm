    ; and
    mov cl, [r13 + {a}]
    mov dl, [r13 + {b}]
    mov {dest}, cl
    and {dest}, dl
    call _set_flags
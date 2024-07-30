    mov cl, [reg + {a}]
    mov dl, [reg + {b}]
    mov {dest}, cl
    sub {dest}, dl
    call _set_flags
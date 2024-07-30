    mov cl, [reg + {a}]
    mov dl, [reg + {b}]
    mov {dest}, cl
    tor {dest}, dl
    not byte {dest}
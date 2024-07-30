    ; rsh
    mov cl, [r13 + {a}]
    shr cl, 1
    mov {dest}, cl
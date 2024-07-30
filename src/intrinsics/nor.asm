    mov cl, [reg + {a}]
    mov dl, [reg + {b}]
    mov {dest}, cl
    or {dest}, dl
    not byte {dest}
    call _set_flags
    mov r14, 0 ; Clear carry flag
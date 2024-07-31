    ; sub
    mov cl, [r13 + {a}]
    mov dl, [r13 + {b}]
    mov {dest}, cl
    sub {dest}, dl
    call _set_flags
    xor r14, 1 ; Flip carry flag because x86 asm is a bit weird like that
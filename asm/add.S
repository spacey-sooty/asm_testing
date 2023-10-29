.text
.global add

add:
        addq %rdi, %rsi
        movq %rsi, %rax
        ret

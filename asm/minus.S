.text
.global minus

minus:
        subq %rdi, %rsi
        movq %rsi, %rax
        ret

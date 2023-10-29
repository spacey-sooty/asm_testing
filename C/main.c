#include <stdio.h>
#include "asm.h"

int main(int argc, char ** argv) {
    int sum = add(1, 2);
    
    printf("%d\n", sum);

    int sub = minus(1, 2);


    printf("%d\n", sub);

    return 0;
}

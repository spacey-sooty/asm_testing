#include <stdio.h>
#include "asm.h"

int main(int argc, char ** argv) {
    int sum = add(1, 2);
    
    printf("%d", sum);

    int sub = minus(1, 2);


    printf("%d", sub);

    return 0;
}

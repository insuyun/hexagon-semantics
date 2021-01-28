#include <stdio.h>
    
int main()
{
    __asm(
    "__begin:"
        "R2 = add(R0, R1); nop; nop; nop;"
    "__end:");
}
#include <stdio.h>
#include <unistd.h>

int main()
{
    int i;
    unsigned int registers[12] = {};
    read(0, registers, 48);
    for (i = 0; i < 10; i++) {
        printf("R%d: %u\n", i, registers[i]);
    }
    printf("C4: %u\n", registers[10]);
    printf("C8: %u\n", registers[11]);
    __asm(
        "{"
        "R10 = %0;"
        "} {"
        "R0 = memw(R10 + #0);"
        "R1 = memw(R10 + #4);"
        "} {"
        "R2 = memw(R10 + #8);"
        "R3 = memw(R10 + #12);"
        "} {"
        "R4 = memw(R10 + #16);"
        "R5 = memw(R10 + #20);"
        "} {"
        "R6 = memw(R10 + #24);"
        "R7 = memw(R10 + #28);"
        "} {"
        "R8 = memw(R10 + #32);"
        "R9 = memw(R10 + #36);"
        "} {"
        "R11 = memw(R10 + #40);"
        "R12 = memw(R10 + #44);"
        "} {"
        "C4 = R11;"
        "} {"
        "C8 = R12;"
        "} {"
        " nop; nop; nop; nop; "
        "} {"
        "R11 = C4;"
        "} {"
        "R12 = C8;"
        "} {"
        "memw(R10 + #40) = R11;"
        "memw(R10 + #48) = R12;"
        "} {"
        "memw(R10 + #0) = R0;"
        "memw(R10 + #4) = R1;"
        "} {"
        "memw(R10 + #8) = R2;"
        "memw(R10 + #16) = R3;"
        "} {"
        "memw(R10 + #20) = R4;"
        "memw(R10 + #24) = R5;"
        "} {"
        "memw(R10 + #32) = R6;"
        "memw(R10 + #36) = R7;"
        "} {"
        "memw(R10 + #40) = R8;"
        "memw(R10 + #44) = R9;"
        "}"
    : : "r" (registers));
    for (i = 0; i < 10; i++) {
        printf("R%d: %u\n", i, registers[i]);
    }
    printf("C4: %u\n", registers[10]);
    printf("C8: %u\n", registers[11]);
}

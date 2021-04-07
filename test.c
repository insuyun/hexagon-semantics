#include <stdio.h>

int main()
{
    int i;
    unsigned int registers[29] = {
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
        0, 1, 2, 3, 4, 5, 6, 7, 8
    };
    for (i = 0; i < 29; i++) {
        scanf("%u", &registers[i]);
    }
    __asm(
        "{"
        "R1 = memw(R29 + #4);"
        "R2 = memw(R29 + #8);"
        "} {"
        "R3 = memw(R29 + #12);"
        "R4 = memw(R29 + #16);"
        "} {"
        "R5 = memw(R29 + #20);"
        "R6 = memw(R29 + #24);"
        "} {"
        "R7 = memw(R29 + #28);"
        "R8 = memw(R29 + #32);"
        "} {"
        "R9 = memw(R29 + #36);"
        "R10 = memw(R29 + #40);"
        "} {"
        "R11 = memw(R29 + #44);"
        "R12 = memw(R29 + #48);"
        "} {"
        "R13 = memw(R29 + #52);"
        "R14 = memw(R29 + #56);"
        "} {"
        "R15 = memw(R29 + #60);"
        "R16 = memw(R29 + #64);"
        "} {"
        "R17 = memw(R29 + #68);"
        "R18 = memw(R29 + #72);"
        "} {"
        "R19 = memw(R29 + #76);"
        "R20 = memw(R29 + #80);"
        "} {"
        "R21 = memw(R29 + #84);"
        "R22 = memw(R29 + #88);"
        "} {"
        "R23 = memw(R29 + #92);"
        "R24 = memw(R29 + #96);"
        "} {"
        "R25 = memw(R29 + #100);"
        "R26 = memw(R29 + #104);"
        "} {"
        "R27 = memw(R29 + #108);"
        "R28 = memw(R29 + #112);"
        "} {"
        "R0 = memw(R29);"
        "}"
    );
  __asm(
    "__begin:"
      "{ R0 = add(R1, R2); }"
    "__end:");    
    __asm(
        "{"
        "memw(R29 + #4) = R1;"
        "memw(R29 + #8) = R2;"
        "} {"
        "memw(R29 + #12) = R3;"
        "memw(R29 + #16) = R4;"
        "} {"
        "memw(R29 + #20) = R5;"
        "memw(R29 + #24) = R6;"
        "} {"
        "memw(R29 + #28) = R7;"
        "memw(R29 + #32) = R8;"
        "} {"
        "memw(R29 + #36) = R9;"
        "memw(R29 + #40) = R10;"
        "} {"
        "memw(R29 + #44) = R11;"
        "memw(R29 + #48) = R12;"
        "} {"
        "memw(R29 + #52) = R13;"
        "memw(R29 + #56) = R14;"
        "} {"
        "memw(R29 + #60) = R15;"
        "memw(R29 + #64) = R16;"
        "} {"
        "memw(R29 + #68) = R17;"
        "memw(R29 + #72) = R18;"
        "} {"
        "memw(R29 + #76) = R19;"
        "memw(R29 + #80) = R20;"
        "} {"
        "memw(R29 + #84) = R21;"
        "memw(R29 + #88) = R22;"
        "} {"
        "memw(R29 + #92) = R23;"
        "memw(R29 + #96) = R24;"
        "} {"
        "memw(R29 + #100) = R25;"
        "memw(R29 + #104) = R26;"
        "} {"
        "memw(R29 + #108) = R27;"
        "memw(R29 + #112) = R28;"
        "} {"
        "memw(R29) = R0;"
        "}"
    );
    // printf("WOW");
    for (i = 0; i < 29; i++) {
        printf("%u\n", registers[i]);
    }
}

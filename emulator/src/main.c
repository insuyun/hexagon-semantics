#include <stdio.h>

int main()
{
  __asm(
    "__begin:"
      "{ R24=sxtb(R23); \
 P3=!cmp.gtu(R14,#466); \
 R15:14=vavguw(R3:2,R9:8):rnd; \
 R5:4=vclip(R27:26,#17); \
 }\n"
    "__end:");  
}

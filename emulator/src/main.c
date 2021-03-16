#include <stdio.h>

int main()
{
  __asm(
    "__begin:"
      "{ r2 = add(r1, r0); r3 = add(r1, r0); r4 = add(r1, r0); r5 = add(r1, r0); }\n"
    "__end:");  
}

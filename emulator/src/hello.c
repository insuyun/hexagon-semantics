#include <stdio.h>

int
main()
{
  printf("Hi\n");
  __asm(
    "__begin:"
      "{ r2 = add(r1, r0);\n"
      "r3 = add(r1, r0);\n"
      "r4 = add(r1, r0);\n"
      "r5 = add(r1, r0);}\n"
    "__end:");
  printf("Bye\n");
}

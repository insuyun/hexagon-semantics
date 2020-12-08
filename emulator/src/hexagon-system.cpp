#include <stdlib.h>
#include <time.h>
#include <iostream>
#include <assert.h>
#include <cstring>

using namespace std;

#include <HexagonTypes.h>
#include <HexagonWrapper.h>

#ifdef WIN32
  #define HEXFUNC							\
      << (strstr(__FUNCTION__,"::") ? strstr(__FUNCTION__,"::") : __FUNCTION__)\
      << ": "
#else
  #define HEXFUNC	__FUNCTION__ << ": "
#endif


#define print_msg(a)	cout << HEXFUNC << a << endl
#define print_err(a)	cerr << HEXFUNC << a << endl

#define CHECK_RC(func)							    \
{									    \
    HEXAPI_Status rc = func;						    \
									    \
    if (rc != HEX_STAT_SUCCESS)						    \
    {									    \
	print_err ("Simulator API Error: " << #func << " returned " << rc); \
    }									    \
}

#define CHECK_FATAL_RC(func, error_code)				\
{									\
    HEXAPI_Status rc = func;						\
									\
    if (rc != HEX_STAT_SUCCESS)						\
    {									\
	print_err ("Simulator API Fatal Error: "			\
				    << #func << " returned " << rc); 	\
	exit (error_code);						\
    }									\
}

#define ISS(x)                         CHECK_RC(iss->x)
#define IF_ISS(x)       { if (iss) { CHECK_RC(iss->x) }}



void PcCallback(void *issPtr) {
  HexagonWrapper *iss = (HexagonWrapper *) issPtr;
  HEX_4u_t r0, r1, r2, r3, r4, r5;
  HEX_4u_t threadId;

  assert(iss->GetCurrentHWThreadNum(&threadId) == HEX_STAT_SUCCESS);
  assert(iss->ReadThreadRegister(threadId, TH_REG_R0, &r0)==HEX_STAT_SUCCESS);
  assert(iss->ReadThreadRegister(threadId, TH_REG_R1, &r1)==HEX_STAT_SUCCESS);
  assert(iss->ReadThreadRegister(threadId, TH_REG_R2, &r2)==HEX_STAT_SUCCESS);
  assert(iss->ReadThreadRegister(threadId, TH_REG_R3, &r3)==HEX_STAT_SUCCESS);
  assert(iss->ReadThreadRegister(threadId, TH_REG_R4, &r4)==HEX_STAT_SUCCESS);
  assert(iss->ReadThreadRegister(threadId, TH_REG_R5, &r5)==HEX_STAT_SUCCESS);

  printf(">>>>Entering:\n\tr0=%d, r1=%d, r2=%d, r3=%d, r4=%d, r5=%d\n", r0, r1, r2, r3, r4, r5);
}

int main(int argc, char **argv) {
  HexagonWrapper *iss;
  HEXAPI_CoreState state;
  HEXAPI_Status rc;
  HEX_4u_t runResult;
  HEX_4u_t hwThreads = 4;
  HEX_4u_t dbgPort;
#define BUFSIZ_STATS 10240
  char buf[BUFSIZ_STATS];
  time_t startTime;
  time_t endtime;
  char *elf;
  char *cpuStr;

  if (argc < 5)
  {
    print_msg ("Syntax:\n" << argv[0] << " -mv<version> -G <port> [<elf>]");
    return -1;
  }

  cpuStr = argv[1];
  dbgPort = atoi (argv[3]);
  elf = argv[4];

  iss = new HexagonWrapper(cpuStr);

  print_msg ("New Hexagon instance: " << cpuStr << " iss=" << hex << iss << dec);

  ISS (ConfigureExecutableBinary(elf));
  ISS (ConfigureCoreFrequency(600000000));
  ISS (ConfigureTimingMode(HEX_TIMING_NODBC));
  ISS (ConfigureExecutableBinary(elf));

  if (dbgPort)
  {
    print_msg ("dbgPort=" << dbgPort);
    ISS (ConfigureRemoteDebug(dbgPort));
  }

  CHECK_FATAL_RC (iss->EndOfConfiguration(), -1);

  ISS (LoadExecutableBinary());

  startTime = time(NULL);

  /*
   *  As long as the simulator is active and at least one thread
   *  is running or in wait state, we will not return from the
   *  'iss->Run' function.
   */

  print_msg ("Run");

  HEX_4u_t start_addr= 0;
  CHECK_FATAL_RC(iss->ReadSymbolValue("__begin", &start_addr), -1);
  iss->AddPCCallback(iss, start_addr, PcCallback);

  HEX_4u_t end_addr;
  CHECK_FATAL_RC(iss->ReadSymbolValue("__end", &end_addr), -1);
  iss->AddPCCallback(iss, end_addr, PcCallback);

  state = iss->Run(&runResult);

  switch (state)
  {
    case HEX_CORE_FINISHED:
      {
        HEX_4u_t modectl;
        HEX_4u_t mask = (1 << hwThreads) - 1;

        print_msg ("core state = HEX_CORE_FINISHED");

        if (dbgPort)
        {
          /*  A debugger was attached, and now the debugger has
           *  exited.
           */
          print_msg ("Debugger has exited\n");
        }

        ISS (ReadGlobalRegister(G_REG_MODECTL, &modectl));

        if ((mask & (modectl | (modectl >> 16))) == 0)
        {
          /*
           *  The Hexagon program has finished.  All hardware threads
           *  are now in 'off' mode, but the AXI slave port is still
           *  enabled.
           *
           *  According to the Hexagon Core HDD, the only way that
           *  the core itself can be woken up from this state is by
           *  toggling the reset pin.  However, the AXI slave port
           *  will still wake up the simulator to service TCM
           *  requests.
           *
           *  IMPORTANT NOTE: An incoming IRQ will NOT wake up the
           *                  core from this state!
           *
           *  Set the pins to idle, but don't assert the actual core
           *  because it's already asserted
           */

          print_msg ("The Hexagon program has exited -- exit code = "
              << runResult);
        }
      }
      break;

    case HEX_CORE_RESET:
      /*
       *  The Hexagon core is in RESET (i.e. HALTED).
       *
       *  All Hexagon threads are off and the AXI slave port
       *  is disabled.
       *
       *  The only way that the core can be woken up from this
       *  state is by toggling the reset pin.  TCM cannot be accessed
       *  when the core is in this state.
       *
       *  IMPORTANT NOTE: An incoming IRQ will NOT wake up the core
       *                  from this state!
       */
      print_msg ("core state = HEX_CORE_RESET");
      break;

    case HEX_CORE_SUCCESS:
    case HEX_CORE_BREAKPOINT:
    case HEX_CORE_ASYNCHRONOUS_BREAK:
    case HEX_CORE_ERROR:
      print_msg ("Hexagon simulation completed; state=" << state);
      break;
  }

  endtime = time(NULL);

  /* Summarize stats */
  ISS (EmitStatistics());

  rc = iss->EmitPerfStatistics (startTime, 0, endtime, 0, buf, sizeof(buf));

  if (rc == HEX_STAT_SUCCESS)
  {
    printf("%s\n", buf);
  }
  return 0;
}

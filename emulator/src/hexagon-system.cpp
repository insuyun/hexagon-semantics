#include <stdlib.h>
#include <time.h>
#include <iostream>
#include <assert.h>
#include <cstring>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>

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

class HexagonSimulator {
    public:
    HexagonWrapper *iss;
    HEX_4u_t reg[29];

    HexagonSimulator(HexagonWrapper *iss) {
        this->iss = iss;

        int urandom = open("/dev/urandom", O_RDONLY);
        for (int i = 0; i < 29; i++) {
            read(urandom, &this->reg[i], sizeof(HEX_4u_t));
        }
        close(urandom);
    }
};

void begin_pc_callback(void *issPtr) {
    HexagonSimulator *sim = (HexagonSimulator *) issPtr;
    HexagonWrapper *iss = sim->iss;
    HEX_4u_t r[32];
    HEX_4u_t threadId;

    assert(iss->GetCurrentHWThreadNum(&threadId) == HEX_STAT_SUCCESS);
    for (int i = 0; i < 29; i++) {
        assert(iss->WriteThreadRegister(threadId, static_cast<thread_register_t>(TH_REG_R0 + i), sim->reg[i])==HEX_STAT_SUCCESS);
    }

    for (int i = 0; i < 32; i++) {
        assert(iss->ReadThreadRegister(threadId, static_cast<thread_register_t>(TH_REG_R0 + i), &r[i])==HEX_STAT_SUCCESS);
    }

    printf(">>>>Entering (Begin):\n\t");
    for (unsigned int i = 0; i < 32; i++) {
        printf("r%u=%u ", i, r[i]);
    }
    printf("\n");
}

void end_pc_callback(void *issPtr) {
    HexagonSimulator *sim = (HexagonSimulator *) issPtr;
    HexagonWrapper *iss = sim->iss;
    HEX_4u_t r[32];
    HEX_4u_t threadId;

    assert(iss->GetCurrentHWThreadNum(&threadId) == HEX_STAT_SUCCESS);

    for (int i = 0; i < 32; i++) {
        assert(iss->ReadThreadRegister(threadId, static_cast<thread_register_t>(TH_REG_R0 + i), &r[i])==HEX_STAT_SUCCESS);
    }

    printf(">>>>Entering (End):\n\t");
    for (unsigned int i = 0; i < 32; i++) {
        printf("r%u=%u ", i, r[i]);
    }
    printf("\n");
}

int main(int argc, char **argv) {
    HexagonWrapper *iss;
    HexagonSimulator *sim;
    HEXAPI_CoreState state;
    HEXAPI_Status rc;
    HEX_4u_t run_result;
    HEX_4u_t hw_threads = 4;
    HEX_4u_t dbgPort;
    #define BUFSIZ_STATS 10240
    char buf[BUFSIZ_STATS];
    time_t start_time;
    time_t end_time;
    char *elf;
    char *cpuStr;

    if (argc < 5)
    {
        print_msg ("Syntax:\n" << argv[0] << " -mv<version> -G <port> [<elf>]");
        return -1;
    }

    cpuStr = argv[1];
    dbgPort = atoi(argv[3]);
    elf = argv[4];

    iss = new HexagonWrapper(cpuStr);
    sim = new HexagonSimulator(iss);

    print_msg ("New Hexagon instance: " << cpuStr << " iss=" << hex << iss << dec);

    CHECK_RC (iss->ConfigureExecutableBinary(elf));
    CHECK_RC (iss->ConfigureCoreFrequency(600000000));
    CHECK_RC (iss->ConfigureTimingMode(HEX_TIMING_NODBC));
    CHECK_RC (iss->ConfigureExecutableBinary(elf));

    if (dbgPort)
    {
        print_msg ("dbgPort=" << dbgPort);
        CHECK_RC (iss->ConfigureRemoteDebug(dbgPort));
    }

    CHECK_FATAL_RC (iss->EndOfConfiguration(), -1);

    CHECK_RC (iss->LoadExecutableBinary());

    start_time = time(NULL);

    /*
    *  As long as the simulator is active and at least one thread
    *  is running or in wait state, we will not return from the
    *  'iss->Run' function.
    */

    print_msg ("Run");

    HEX_4u_t start_addr= 0;
    CHECK_FATAL_RC(iss->ReadSymbolValue("__begin", &start_addr), -1);
    iss->AddPCCallback(sim, start_addr, begin_pc_callback);

    HEX_4u_t end_addr;
    CHECK_FATAL_RC(iss->ReadSymbolValue("__end", &end_addr), -1);
    iss->AddPCCallback(sim, end_addr, end_pc_callback);

    state = iss->Run(&run_result);

    switch (state)
    {
        case HEX_CORE_FINISHED:
            {
                HEX_4u_t modectl;
                HEX_4u_t mask = (1 << hw_threads) - 1;

                print_msg ("core state = HEX_CORE_FINISHED");

                if (dbgPort)
                {
                /*  A debugger was attached, and now the debugger has
                *  exited.
                */
                print_msg ("Debugger has exited\n");
                }

                CHECK_RC (iss->ReadGlobalRegister(G_REG_MODECTL, &modectl));

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
                    << run_result);
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

    end_time = time(NULL);

    /* Summarize stats */
    CHECK_RC (iss->EmitStatistics());

    rc = iss->EmitPerfStatistics (start_time, 0, end_time, 0, buf, sizeof(buf));

    if (rc == HEX_STAT_SUCCESS)
    {
        printf("%s\n", buf);
    }
    return 0;
}

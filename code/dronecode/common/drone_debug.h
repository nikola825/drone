#pragma once

#include<Arduino.h>

#define DBGOUT Serial

#define DBGLEVEL 0

#ifdef DBGOUT

#define DBG_PRINTLN(LEVEL, ...) if(LEVEL<=DBGLEVEL){ DBGOUT.println(__VA_ARGS__);};

#define DBG_PRINTVAR(LEVEL, VAR)\
    if(LEVEL<=DBGLEVEL){\
    DBGOUT.print(#VAR"=");\
    DBGOUT.println(VAR);\
    }

#else

#define DBG_PRINTLN(LEVEL,...)

#define DBG_PRINTVAR(LEVEL,VAR)

#endif

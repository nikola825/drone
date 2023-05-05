#include "commandimpl.h"
#include "commands.h"

void (*(commands[256]))(uint8_t[8])={
    COMMAND_FUNCTION_DEF0(halt_command)
        halt(0);
    },
    COMMAND_FUNCTION_DEF0(start_command)

    },
    COMMAND_FUNCTION_DEF0(stop_command)
    },
    COMMAND_FUNCTION_DEF0(heartbeat_command)
    },
    COMMAND_FUNCTION_DEF1(read_storage_command, uint16_t, address)
    },
    COMMAND_FUNCTION_DEF3(write_storage_command, uint16_t, address, uint8_t, length, uint32_t, value)        
    }
};
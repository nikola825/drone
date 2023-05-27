#include "commandimpl.h"
#include "commands.h"
#include "storage.h"
#include "motors.h"

void (*(commands[256]))(uint8_t[8])={
    COMMAND_FUNCTION_DEF0(halt_command)
        halt(HALT_BY_USER_INPUT);
    },
    COMMAND_FUNCTION_DEF0(start_command)

    },
    COMMAND_FUNCTION_DEF0(stop_command)
        stop_motors();
    },
    COMMAND_FUNCTION_DEF0(heartbeat_command)
    },
    COMMAND_FUNCTION_DEF1(read_storage_command, uint16_t, address)
    },
    COMMAND_FUNCTION_DEF3(write_storage_command, uint16_t, address, uint8_t, length, uint32_t, value)

        uint8_t *cursor = &(storage[address]);
        uint16_t *ti = &thrust_input;

        DBG_PRINTLN(2, (uint32_t)cursor);
        DBG_PRINTLN(2, (uint32_t)ti);

        for(uint8_t i=0;i<length;i++)
        {
            *cursor = value&0xff;
            value >>= 8;
            cursor++;
        }
    }
};
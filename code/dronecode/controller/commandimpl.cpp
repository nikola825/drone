#include "commandimpl.h"
#include "commands.h"
#include "storage.h"
#include "motors.h"
#include "motion.h"

void (*(commands[256]))(uint8_t[8])={
    COMMAND_FUNCTION_DEF0(halt_command)
        halt(HALT_ERROR::USER_HALT);
    },
    COMMAND_FUNCTION_DEF0(start_command)
        start_motors();
    },
    COMMAND_FUNCTION_DEF0(stop_command)
        DBG_PRINTLN(1, "Stopping motors by command");
        stop_motors();
    },
    COMMAND_FUNCTION_DEF0(heartbeat_command)
    },
    COMMAND_FUNCTION_DEF1(read_storage_command, uint16_t, address)
    },
    COMMAND_FUNCTION_DEF3(write_storage_command, uint16_t, address, uint8_t, length, uint32_t, value)
        uint8_t *cursor = &(storage[address]);

        for(uint8_t i=0;i<length;i++)
        {
            *cursor = value&0xff;
            value >>= 8;
            cursor++;
        }
    },
    COMMAND_FUNCTION_DEF1(set_mpu_rate, uint8_t, rate)
        mpu_set_rate(rate);
    },
    COMMAND_FUNCTION_DEF1(set_mpu_dlpf, uint8_t, dlpf)
        mpu_set_dlpf(dlpf);
    },
};
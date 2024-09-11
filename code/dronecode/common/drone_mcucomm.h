//
// Created by nidzo on 6/10/24.
//

#ifndef CODE_DRONE_MCUCOMM_H
#define CODE_DRONE_MCUCOMM_H
#include <Arduino.h>

struct MCUPacket
{
    //2-FL
    //1-RR
    //0-FR
    //3-RL
    uint16_t motor0: 10;
    uint16_t motor1:10;
    uint16_t motor2: 10;
    uint16_t motor3:10;
    uint32_t padding:24;
};


/*
 * uint16_t front_left: 10;
    uint16_t front_right:10;
    uint16_t rear_left: 10;
    uint16_t rear_right:10;
 */
static_assert(sizeof(MCUPacket) == 8, "MCU comm packet must be 8 bytes");
#define INTER_MCU_COMM_TIMEOUT 100
#endif //CODE_DRONE_MCUCOMM_H

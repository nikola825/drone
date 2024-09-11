//
// Created by nidzo on 6/10/24.
//

#ifndef CODE_DRONE_ERROR_H
#define CODE_DRONE_ERROR_H

#include "common_stuff.h"
#include "drone_debug.h"
extern bool halted;
extern void stop_motors();

constexpr int ERROR_FLASH_PERIOD = 300;
constexpr int ERROR_FLASH_DELAY = 1000;

namespace HALT_ERROR
{
    enum HALT_ERROR
    {
        SERVO_INPUT_OUT_OF_RANGE = 2,
        INTER_MCU_TIMEOUT = 3,
        WIRE_FAIL = 4,
        MPU_FAIL = 5,
        BUFFER_OVERFLOW=6,
        BUFFER_UNDEFLOW=7,
        RADIO_FAIL=8,
        USER_HALT=9,
        CONTROL_INPUT_OUT_OF_RANGE=10,
        THRUST_INPUT_OUT_OF_RANGE=11
    };
}

/*constexpr uint8_t HALT_BUFFER_OVERFLOW = 1;
constexpr uint8_t HALT_BUFFER_UNDERFLOW = 2;
constexpr uint8_t HALT_MOTOR_INPUT_OUT_OF_RANGE = 3;
constexpr uint8_t HALT_THRUST_INPUT_OUT_OF_RANGE = 4;
constexpr uint8_t HALT_BY_USER_INPUT = 7;
constexpr uint8_t HALT_WIRE_FAIL = 8;
constexpr uint8_t HALT_MPU_FAIL = 9;
constexpr uint8_t HALT_EEPROM_FAIL = 10;
constexpr uint8_t HALT_RADIO_FAIL = 11;*/

extern void pullup_ERROR();
extern void pulldown_ERROR();
static uint8_t global_error;

static inline void flash_error()
{
    wdt_reset();
    for(int i=0; i<global_error; i++)
    {
        wdt_reset();
        pullup_ERROR();
        wdt_reset();
        delay(ERROR_FLASH_PERIOD/2);
        wdt_reset();
        delay(ERROR_FLASH_PERIOD/2);
        wdt_reset();
        pulldown_ERROR();
        wdt_reset();
        delay(ERROR_FLASH_PERIOD/2);
        wdt_reset();
        delay(ERROR_FLASH_PERIOD/2);
        wdt_reset();
    }
    for(int i=0;i<10;i++)
    {
        wdt_reset();
        delay(ERROR_FLASH_DELAY/10);
    }
    wdt_reset();
}

static inline void halt(uint8_t error)
{
    DBG_PRINTLN(0, "Halting");
    DBG_PRINTVAR(0, error);
    halted=true;
    stop_motors();
    global_error = error;
    while (1)
    {
        wdt_reset();
        flash_error();
    }
}

#endif //CODE_DRONE_ERROR_H

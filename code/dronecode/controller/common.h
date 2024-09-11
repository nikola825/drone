//
// Created by nidzo on 6/10/24.
//

#ifndef CODE_COMMON_H
#define CODE_COMMON_H

#include <Arduino.h>
#include "atmelports.h"
#include "drone_error.h"
#include "drone_debug.h"
#include "drone_mcucomm.h"

constexpr uint16_t THRUST_INPUT_RANGE = 3000;

extern bool watchdog_enabled;
extern bool motors_enabled;

void pullup_BLUE();
void pulldown_BLUE();
void pullup_GREEN();
void pulldown_GREEN();

#endif //CODE_COMMON_H

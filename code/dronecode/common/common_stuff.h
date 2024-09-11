//
// Created by nidzo on 6/10/24.
//

#ifndef CODE_COMMON_STUFF_H
#define CODE_COMMON_STUFF_H
#include <Arduino.h>
#include <avr/wdt.h>
constexpr uint8_t WATCHDOG_TIMEOUT = WDTO_250MS;
extern bool watchdog_enabled;

#endif //CODE_COMMON_STUFF_H

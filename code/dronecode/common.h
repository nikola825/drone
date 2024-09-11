#pragma once
#include<Arduino.h>
#include<Servo.h>
#include "debug.h"
#include <avr/wdt.h>

constexpr uint8_t HALT_BUFFER_OVERFLOW = 1;
constexpr uint8_t HALT_BUFFER_UNDERFLOW = 2;
constexpr uint8_t HALT_MOTOR_INPUT_OUT_OF_RANGE = 3;
constexpr uint8_t HALT_THRUST_INPUT_OUT_OF_RANGE = 4;
constexpr uint8_t HALT_SERVO_INPUT_OUT_OF_RANGE = 5;
constexpr uint8_t HALT_BY_USER_INPUT = 7;
constexpr uint8_t WIRE_FAIL = 8;
constexpr uint8_t MPU_FAIL = 9;
constexpr uint8_t EEPROM_FAIL = 10;
constexpr uint8_t RADIO_FAIL = 11;



extern bool halted;

extern uint8_t global_error;

extern bool watchdog_enabled;

void halt(uint8_t error);


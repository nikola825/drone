#pragma once

#include "common.h"

constexpr int SERVO_MIN = 1000;
constexpr int SERVO_MAX = 2000;
constexpr int SERVO_RANGE = SERVO_MAX - SERVO_MIN;

constexpr int FAST_STOP_DELAY_TIME_MS = 100;
constexpr uint16_t THRUST_INPUT_RANGE = SERVO_RANGE*4;

void init_motors();

void stop_motors();

void drive(MCUPacket &packet);
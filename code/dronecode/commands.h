#pragma once
#include "common.h"
#include "commandimpl.h"

constexpr int HALT_BUFFER_OVERFLOW = 1;
constexpr int HALT_BUFFER_UNDERFLOW = 2;

void process_commands(HardwareSerial &serial);
void halt(int error);
void setup_commands();
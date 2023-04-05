#pragma once
#include "common.h"

extern HardwareSerial &bluetooth_port;

void bt_init();

void bt_at_terminal();
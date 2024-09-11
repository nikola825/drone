#pragma once
#include "common.h"

#define bluetooth_port Serial1
#define BT_DEBUG

void bt_init();

void bt_at_terminal();

#ifdef BT_DEBUG

void bt_send_int(uint8_t id, uint32_t value);

void bt_send_float(uint8_t id, float value);

#else

#define bt_send_int(uint8_t, uint16_t)
#define bt_send_float(uint8_t, uint16_t)

#endif
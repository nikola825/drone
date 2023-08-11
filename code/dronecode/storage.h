#pragma once
#include "common.h"

extern uint8_t storage[1024];

#ifndef STOREDVAR

#define STOREDVAR(TYPE,NAME,PREVIOUS) \
    extern TYPE &NAME;

#endif

extern uint32_t& storage_start;

// navigation inputs
STOREDVAR(uint16_t, thrust_input, storage_start);
STOREDVAR(int16_t, yaw_input, thrust_input);
STOREDVAR(int16_t, pitch_input, yaw_input);
STOREDVAR(int16_t, roll_input, pitch_input);

// PID parameters
STOREDVAR(int16_t, yaw_kp, roll_input);
STOREDVAR(int16_t, yaw_ki, yaw_kp);
STOREDVAR(int16_t, yaw_kd, yaw_ki);
STOREDVAR(int16_t, pitch_kp, yaw_kd);
STOREDVAR(int16_t, pitch_ki, pitch_kp);
STOREDVAR(int16_t, pitch_kd, pitch_ki);
STOREDVAR(int16_t, roll_kp, pitch_kd);
STOREDVAR(int16_t, roll_ki, roll_kp);
STOREDVAR(int16_t, roll_kd, roll_ki);

// motor inputs
STOREDVAR(uint16_t, motor_thrust, roll_kd);
STOREDVAR(int16_t, motor_yaw, motor_thrust);
STOREDVAR(int16_t, motor_pitch, motor_yaw);
STOREDVAR(int16_t, motor_roll, motor_pitch);

void init_storage();
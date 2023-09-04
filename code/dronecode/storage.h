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
STOREDVAR(uint16_t, yaw_velocity_kp, roll_input);
STOREDVAR(uint16_t, yaw_velocity_ki, yaw_velocity_kp);
STOREDVAR(uint16_t, yaw_velocity_kd, yaw_velocity_ki);
STOREDVAR(uint16_t, pitch_velocity_kp, yaw_velocity_kd);
STOREDVAR(uint16_t, pitch_velocity_ki, pitch_velocity_kp);
STOREDVAR(uint16_t, pitch_velocity_kd, pitch_velocity_ki);
STOREDVAR(uint16_t, roll_velocity_kp, pitch_velocity_kd);
STOREDVAR(uint16_t, roll_velocity_ki, roll_velocity_kp);
STOREDVAR(uint16_t, roll_velocity_kd, roll_velocity_ki);

STOREDVAR(uint16_t, pitch_angle_kp, roll_velocity_kd);
STOREDVAR(uint16_t, pitch_angle_ki, pitch_angle_kp);
STOREDVAR(uint16_t, pitch_angle_kd, pitch_angle_ki);

STOREDVAR(uint16_t, roll_angle_kp, pitch_angle_kd);
STOREDVAR(uint16_t, roll_angle_ki, roll_angle_kp);
STOREDVAR(uint16_t, roll_angle_kd, roll_angle_ki);

// motor inputs
STOREDVAR(uint16_t, motor_thrust, roll_angle_kd);
STOREDVAR(int16_t, motor_yaw, motor_thrust);
STOREDVAR(int16_t, motor_pitch, motor_yaw);
STOREDVAR(int16_t, motor_roll, motor_pitch);

void init_storage();
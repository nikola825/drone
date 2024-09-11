//
// Created by nidzo on 7/30/24.
//
#include "common.h"
#include "storage.h"
bool motors_enabled;
MCUPacket pack;

MAKE_PIN(Sending, H, 7, 1);
MAKE_PIN(Receiving, H, 6, 0);
MAKE_PIN(DoneSending, H, 5, 1);
MAKE_PIN(DoneReceiving, H, 4, 0);

#define MP0 F
#define MP1 K
#define MP2 A
#define MP3 L
#define MP4 C

void do_send(MCUPacket const& packet)
{
    uint8_t *packed_packet = (uint8_t*)(&packet);

    PORT(MP0) = packed_packet[0];
    PORT(MP1) = packed_packet[1];
    PORT(MP2) = packed_packet[2];
    PORT(MP3) = packed_packet[3];
    PORT(MP4) = packed_packet[4];

    pulldown_Sending();
    while(read_Receiving());
    pullup_Sending();
    pulldown_DoneSending();
    while(read_DoneReceiving());
    pullup_DoneSending();
}

void init_motors()
{
    DDR(MP0) = 0xff;
    DDR(MP1) = 0xff;
    DDR(MP2) = 0xff;
    DDR(MP3) = 0xff;
    DDR(MP4) = 0xff;
    PORTH |= 0xf0;
    DDR(H) |= 0xa0;

    motors_enabled = false;
    pack.motor0=pack.motor1=pack.motor2=pack.motor3=0;
    do_send(pack);
}
void stop_motors()
{
    DBG_PRINTLN(1, "Stopping motors");
    motors_enabled = false;
    pulldown_GREEN();
}
void start_motors()
{
    DBG_PRINTLN(1, "Starting motors");
    pack.motor0=pack.motor1=pack.motor2=pack.motor3=0;
    motors_enabled = true;
    pullup_GREEN();
}
void drive_motors()
{
    if(!motors_enabled)
    {
        return;
    }
    DBG_PRINTVAR(2, motor_roll);
    DBG_PRINTVAR(2, motor_pitch);
    DBG_PRINTVAR(2, motor_yaw);

    const int16_t command_limit = motor_thrust >> 2;

    if (motor_roll > command_limit || motor_roll < -command_limit ||
        motor_pitch > command_limit || motor_pitch < -command_limit ||
        motor_yaw > command_limit || motor_yaw < -command_limit)
    {
        halt(HALT_ERROR::CONTROL_INPUT_OUT_OF_RANGE);
        return;
    }

    if (motor_thrust > THRUST_INPUT_RANGE)
    {
        halt(HALT_ERROR::THRUST_INPUT_OUT_OF_RANGE);
        return;
    }


    uint16_t front_left = (motor_thrust + motor_roll + motor_pitch + motor_yaw) / 4;
    uint16_t front_right = (motor_thrust - motor_roll + motor_pitch - motor_yaw) / 4;
    uint16_t rear_left = (motor_thrust + motor_roll - motor_pitch - motor_yaw) / 4;
    uint16_t rear_right = (motor_thrust - motor_roll - motor_pitch + motor_yaw) / 4;

    //2-FL
    //1-RR
    //0-FR
    //3-RL
    pack.motor0 = front_left;
    pack.motor1 = rear_right;
    pack.motor2 = front_right;
    pack.motor3 = rear_left;

    do_send(pack);
}
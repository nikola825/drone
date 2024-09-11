//
// Created by nidzo on 7/30/24.
//
#include "common.h"
#include "storage.h"

constexpr int STOP_DECAY_PERCENTAGE = 70;
constexpr int FAST_STOP_DELAY_TIME_MS = 100;

bool motors_enabled;

/*MAKE_PIN(Sending, H, 7, 1);
MAKE_PIN(Receiving, H, 6, 0);
MAKE_PIN(DoneSending, H, 5, 1);
MAKE_PIN(DoneReceiving, H, 4, 0);*/

#define DSHOT_BITBANG_BIT(IOREG, IOREGV, BIT, SOURCE) \
    asm volatile ( \
    "ldi r24, " IOREGV "\n" \
    "ldi r23, 0\n" \
    "sts " IOREG ", r24\n" \
    "sbrc  %0, " BIT "\n" \
    "jmp L1_%=\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "sts " IOREG ", r23\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "jmp Lk_%=\n" \
    "L1_%=: sts " IOREG ", r24\n" \
    "nop\n" \
    "nop\n" \
    "nop\n"\
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "sts " IOREG ", r23\n" \
    "nop\n" \
    "nop\n" \
    "Lk_%=:nop\n" \
    : \
    :"r"(SOURCE) \
    :"r24", "r23" \
    );

#define DSHOT_BITBANG_BIT_LOREG(IOREG, REGBIT, BIT, SOURCE) \
    asm volatile ( \
    "ldi r24, " REGBIT "\n" \
    "ldi r23, 0\n" \
    "sbi " IOREG ", " REGBIT "\n" \
    "sbrc  %0, " BIT "\n" \
    "jmp L1_%=\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "cbi " IOREG ", " REGBIT "\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "jmp Lk_%=\n" \
    "L1_%=: sbi " IOREG ", " REGBIT "\n" \
    "nop\n" \
    "nop\n" \
    "nop\n"\
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "nop\n" \
    "cbi " IOREG ", " REGBIT "\n" \
    "nop\n" \
    "nop\n" \
    "Lk_%=:nop\n" \
    : \
    :"r"(SOURCE) \
    :"r24", "r23" \
    );
#define DSHOT_BITBANG_PORT(IOREG, IOREGV) \
    DSHOT_BITBANG_BIT(IOREG, IOREGV, "7", high); \
    DSHOT_BITBANG_BIT(IOREG, IOREGV, "6", high); \
    DSHOT_BITBANG_BIT(IOREG, IOREGV, "5", high); \
    DSHOT_BITBANG_BIT(IOREG, IOREGV, "4", high); \
    DSHOT_BITBANG_BIT(IOREG, IOREGV, "3", high); \
    DSHOT_BITBANG_BIT(IOREG, IOREGV, "2", high); \
    DSHOT_BITBANG_BIT(IOREG, IOREGV, "1", high); \
    DSHOT_BITBANG_BIT(IOREG, IOREGV, "0", high); \
    DSHOT_BITBANG_BIT(IOREG, IOREGV, "7", low); \
    DSHOT_BITBANG_BIT(IOREG, IOREGV, "6", low); \
    DSHOT_BITBANG_BIT(IOREG, IOREGV, "5", low); \
    DSHOT_BITBANG_BIT(IOREG, IOREGV, "4", low); \
    DSHOT_BITBANG_BIT(IOREG, IOREGV, "3", low); \
    DSHOT_BITBANG_BIT(IOREG, IOREGV, "2", low); \
    DSHOT_BITBANG_BIT(IOREG, IOREGV, "1", low); \
    DSHOT_BITBANG_BIT(IOREG, IOREGV, "0", low);

#define DSHOT_BITBANG_PORT_LOREG(IOREG, REGBIT) \
    DSHOT_BITBANG_BIT_LOREG(IOREG, REGBIT, "7", high); \
    DSHOT_BITBANG_BIT_LOREG(IOREG, REGBIT, "6", high); \
    DSHOT_BITBANG_BIT_LOREG(IOREG, REGBIT, "5", high); \
    DSHOT_BITBANG_BIT_LOREG(IOREG, REGBIT, "4", high); \
    DSHOT_BITBANG_BIT_LOREG(IOREG, REGBIT, "3", high); \
    DSHOT_BITBANG_BIT_LOREG(IOREG, REGBIT, "2", high); \
    DSHOT_BITBANG_BIT_LOREG(IOREG, REGBIT, "1", high); \
    DSHOT_BITBANG_BIT_LOREG(IOREG, REGBIT, "0", high); \
    DSHOT_BITBANG_BIT_LOREG(IOREG, REGBIT, "7", low); \
    DSHOT_BITBANG_BIT_LOREG(IOREG, REGBIT, "6", low); \
    DSHOT_BITBANG_BIT_LOREG(IOREG, REGBIT, "5", low); \
    DSHOT_BITBANG_BIT_LOREG(IOREG, REGBIT, "4", low); \
    DSHOT_BITBANG_BIT_LOREG(IOREG, REGBIT, "3", low); \
    DSHOT_BITBANG_BIT_LOREG(IOREG, REGBIT, "2", low); \
    DSHOT_BITBANG_BIT_LOREG(IOREG, REGBIT, "1", low); \
    DSHOT_BITBANG_BIT_LOREG(IOREG, REGBIT, "0", low);

void dshot_bitbang_d4(uint8_t high, uint8_t low)
{
    asm("cli");
    DSHOT_BITBANG_PORT_LOREG("0x0b", "4");
    asm("sei");
}

void dshot_bitbang_d5(uint8_t high, uint8_t low)
{
    asm("cli");
    DSHOT_BITBANG_PORT_LOREG("0x0b", "5");
    asm("sei");
}

void dshot_bitbang_d6(uint8_t high, uint8_t low)
{
    asm("cli");
    DSHOT_BITBANG_PORT_LOREG("0x0b", "6");
    asm("sei");
}

void dshot_bitbang_d7(uint8_t high, uint8_t low)
{
    asm("cli");
    DSHOT_BITBANG_PORT_LOREG("0x0b", "7");
    asm("sei");
}

uint16_t dshot_crc(uint16_t command)
{
    command <<= 1;
    uint16_t crc = (((command >> 0) & 0x0f) ^ ((command >> 4) & 0x0f) ^ ((command >> 8) & 0x0f)) & 0x0f;
    command = (command << 4) | crc;
    return command;
}

void dshot_bitbang_motor0(uint16_t command)
{
    command = dshot_crc(command);
    uint8_t high = (command >> 8) & 0xff;
    uint8_t low = command & 0xff;
    dshot_bitbang_d4(high, low);
}

void dshot_bitbang_motor1(uint16_t command)
{
    command = dshot_crc(command);
    uint8_t high = (command >> 8) & 0xff;
    uint8_t low = command & 0xff;
    dshot_bitbang_d5(high, low);
}

void dshot_bitbang_motor2(uint16_t command)
{
    command = dshot_crc(command);
    uint8_t high = (command >> 8) & 0xff;
    uint8_t low = command & 0xff;
    dshot_bitbang_d6(high, low);
}

void dshot_bitbang_motor3(uint16_t command)
{
    command = dshot_crc(command);
    uint8_t high = (command >> 8) & 0xff;
    uint8_t low = command & 0xff;
    dshot_bitbang_d7(high, low);
}

void send_thrustq(uint16_t motor0, uint16_t motor1, uint16_t motor2, uint16_t motor3)
{
    dshot_bitbang_motor0(motor0 == 0 ? 0 : motor0 * 2 + 48);
    dshot_bitbang_motor1(motor1 == 0 ? 0 : motor1 * 2 + 48);
    dshot_bitbang_motor2(motor2 == 0 ? 0 : motor2 * 2 + 48);
    dshot_bitbang_motor3(motor3 == 0 ? 0 : motor3 * 2 + 48);
}

void init_motors()
{
    DDRD |= 0xf0;

    motors_enabled = false;
    motor_thrust = 0;
    thrust_input = 0;
}

void stop_motors()
{
    DBG_PRINTLN(1, "Stopping motors");
    motors_enabled = false;

    uint32_t thrust_target = motor_thrust / 4;

    while (thrust_target > 0)
    {
        thrust_target = (thrust_target * STOP_DECAY_PERCENTAGE) / 100;
        if (thrust_target <= 25)
        {
            thrust_target = 0;
        }

        send_thrustq(thrust_target, thrust_target, thrust_target, thrust_target);


        wdt_reset();
        delay(FAST_STOP_DELAY_TIME_MS);
    }

    send_thrustq(0, 0, 0, 0);
    motor_thrust = 0;
    thrust_input = 0;

    pullup_YELLOW();
    pulldown_GREEN();
}

void start_motors()
{
    DBG_PRINTLN(1, "Starting motors");
    motors_enabled = true;
    pullup_GREEN();
    pulldown_YELLOW();
    pulldown_RED();
    motor_thrust = 0;
    thrust_input = 0;
}

void drive_motors()
{
    if (!motors_enabled)
    {
        send_thrustq(0, 0, 0, 0);
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

    send_thrustq(front_right, rear_left, front_left, rear_right);
}
//
// Created by nidzo on 6/10/24.
//
#include<Servo.h>
#include "common.h"
#include "motors.h"

constexpr int MOTOR_COUNT = 4;

constexpr int STOP_DECAY_PERCENTAGE = 70;

uint16_t motor0, motor1, motor2, motor3;
extern bool halted;


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

void dshot_bitbang_j0(uint8_t high, uint8_t low)
{
    asm("cli");
    DSHOT_BITBANG_PORT("0x105", "1");
    asm("sei");
}

void dshot_bitbang_j1(uint8_t high, uint8_t low)
{
    asm("cli");
    DSHOT_BITBANG_PORT("0x105", "2");
    asm("sei");
}

void dshot_bitbang_a7(uint8_t high, uint8_t low)
{
    asm("cli");
    DSHOT_BITBANG_PORT_LOREG("0x02", "7");
    asm("sei");
}

void dshot_bitbang_a6(uint8_t high, uint8_t low)
{
    asm("cli");
    DSHOT_BITBANG_PORT_LOREG("0x02", "6");
    asm("sei");
}

uint16_t dshot_crc(uint16_t command)
{
    uint16_t telem = 0;
    if(command<48 && command>0) telem=1;
    command<<=1;
    command|=telem;
    uint16_t crc = (((command>>0) & 0x0f) ^ ((command>>4) & 0x0f) ^ ((command>>8) & 0x0f))&0x0f;
    command = (command << 4) | crc;
    return command;
}


uint16_t dshot_crc2(uint16_t command)
{
    command<<=1;
    uint16_t crc = (((command>>0) & 0x0f) ^ ((command>>4) & 0x0f) ^ ((command>>8) & 0x0f))&0x0f;
    command = (command << 4) | crc;
    return command;
}

void dshot_bitbang_motor0(uint16_t command)
{
    command = dshot_crc(command);
    uint8_t high = (command>>8)&0xff;
    uint8_t low = command&0xff;
    dshot_bitbang_j0(high, low);
}

void dshot_bitbang_motor1(uint16_t command)
{
    command = dshot_crc(command);
    uint8_t high = (command>>8)&0xff;
    uint8_t low = command&0xff;
    dshot_bitbang_j1(high, low);
}

void dshot_bitbang_motor2(uint16_t command)
{
    command = dshot_crc(command);
    uint8_t high = (command>>8)&0xff;
    uint8_t low = command&0xff;
    dshot_bitbang_a7(high, low);
}

void dshot_bitbang_motor3(uint16_t command)
{
    command = dshot_crc(command);
    uint8_t high = (command>>8)&0xff;
    uint8_t low = command&0xff;
    dshot_bitbang_a6(high, low);
}

void init_motors()
{
    digitalWrite(14, LOW);
    digitalWrite(15, LOW);
    digitalWrite(28, LOW);
    digitalWrite(29, LOW);
    pinMode(14, OUTPUT);
    pinMode(15, OUTPUT);
    pinMode(28, OUTPUT);
    pinMode(29, OUTPUT);
    motor1 = motor0 = motor3 = motor2 = 0;
}

void stop_motors()
{
    DBG_PRINTLN(2, "Stopping");
    uint32_t thrust_target = (motor1+motor0+motor3+motor2) / 2;

    if(watchdog_enabled)
    {
        wdt_disable();
    }

    while (thrust_target > 0)
    {
        thrust_target = (thrust_target * STOP_DECAY_PERCENTAGE) / 100;

        if (thrust_target <= 50)
        {
            thrust_target = 0;
        }

        dshot_bitbang_motor0(48+thrust_target);
        dshot_bitbang_motor1(48+thrust_target);
        dshot_bitbang_motor2(48+thrust_target);
        dshot_bitbang_motor3(48+thrust_target);

        if (watchdog_enabled)
        {
            wdt_reset();
        }
        delay(FAST_STOP_DELAY_TIME_MS);
    }
    motor1 = motor0 = motor3 = motor2 = 0;
    if(watchdog_enabled)
    {
        wdt_enable(WATCHDOG_TIMEOUT);
    }

    dshot_bitbang_motor0(0);
    dshot_bitbang_motor1(0);
    dshot_bitbang_motor2(0);
    dshot_bitbang_motor3(0);
}

void drive(MCUPacket &packet)
{
    if (halted)
    {
        stop_motors();
        return;
    }

    motor0 = packet.motor0;
    motor1 = packet.motor1;
    motor2 = packet.motor2;
    motor3 = packet.motor3;

    if (motor0 > SERVO_RANGE ||
        motor1 > SERVO_RANGE ||
        motor2 > SERVO_RANGE ||
        motor3 > SERVO_RANGE)
    {
        halt(HALT_ERROR::SERVO_INPUT_OUT_OF_RANGE);
        return;
    }

    if(motor2 > 0)
    {
        dshot_bitbang_motor0(47+motor2*2);
    }
    else
    {
        dshot_bitbang_motor0(0);
    }

    if(motor1 > 0)
    {
        dshot_bitbang_motor1(47+motor1*2);
    }
    else
    {
        dshot_bitbang_motor1(0);
    }

    if(motor0 > 0)
    {
        dshot_bitbang_motor2(47+motor0*2);
    }
    else
    {
        dshot_bitbang_motor2(0);
    }

    if(motor3 > 0)
    {
        dshot_bitbang_motor3(47+motor3*2);
    }
    else
    {
        dshot_bitbang_motor3(0);
    }
}
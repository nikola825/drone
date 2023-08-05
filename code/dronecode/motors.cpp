#include "common.h"
#include "motors.h"
#include "storage.h"
#include "commands.h"

constexpr int FL_PIN = A0;
constexpr int FL_GND = A2;

constexpr int FR_PIN = A3;
constexpr int FR_GND = A5;

constexpr int RL_PIN = 2;
constexpr int RL_GND = 4;

constexpr int RR_PIN = 5;
constexpr int RR_GND = 7;

constexpr int FANCY_LOW = 6;
constexpr int FANCY_HIGH = 15;

constexpr int MOTOR_COUNT = 4;

constexpr int grounds[MOTOR_COUNT] = {FL_GND, FR_GND, RR_GND, RL_GND};

constexpr int STOP_DECAY_PERCENTAGE = 70;

Servo fl;
Servo fr;
Servo rr;
Servo rl;

constexpr Servo *all_motors[MOTOR_COUNT] = {&fl, &fr, &rr, &rl};

void init_motors()
{
    for (int i = 0; i < 4; i++)
    {
        digitalWrite(grounds[i], LOW);
        pinMode(grounds[i], OUTPUT);
        digitalWrite(grounds[i], LOW);
    }

    fl.attach(FL_PIN, SERVO_MIN, SERVO_MAX);
    fl.write(0);

    fr.attach(FR_PIN, SERVO_MIN, SERVO_MAX);
    fr.write(0);

    rr.attach(RR_PIN, SERVO_MIN, SERVO_MAX);
    rr.write(0);

    rl.attach(RL_PIN, SERVO_MIN, SERVO_MAX);
    rl.write(0);

    motor_thrust = 0;
    motor_pitch = 0;
    motor_yaw = 0;
    motor_roll = 0;
}

void stop_motors()
{
    constexpr int delay_time = FAST_STOP_DELAY_TIME_MS;
    DBG_PRINTLN(2, "Stopping");
    DBG_PRINTVAR(2, delay_time);
    uint32_t thrust_target = motor_thrust / 4;

    while (thrust_target > 0)
    {
        thrust_target = (thrust_target * STOP_DECAY_PERCENTAGE) / 100;
        if (delay_time == 0 || thrust_target <= 50)
        {
            thrust_target = 0;
        }
        for (auto &motor: all_motors)
        {
            motor->writeMicroseconds(SERVO_MIN + thrust_target);
        }
        if (delay_time)
        {
            delay(delay_time);
        }
    }

    for (auto &motor: all_motors)
    {
        motor->writeMicroseconds(SERVO_MIN);
    }

    thrust_input = 0;
    motor_thrust = 0;
}

void drive()
{
    DBG_PRINTVAR(2, motor_thrust);
    DBG_PRINTVAR(2, motor_roll);
    DBG_PRINTVAR(2, motor_pitch);
    DBG_PRINTVAR(2, motor_yaw);
    if (halted || motor_thrust == 0)
    {
        stop_motors();
        return;
    }

    if (motor_roll > MOTOR_INPUT_RANGE || motor_roll < -MOTOR_INPUT_RANGE ||
        motor_pitch > MOTOR_INPUT_RANGE || motor_pitch < -MOTOR_INPUT_RANGE ||
        motor_yaw > MOTOR_INPUT_RANGE || motor_yaw < -MOTOR_INPUT_RANGE)
    {
        halt(HALT_MOTOR_INPUT_OUT_OF_RANGE);
        return;
    }

    if (motor_thrust > THRUST_INPUT_RANGE)
    {
        halt(HALT_THRUST_INPUT_OUT_OF_RANGE);
        return;
    }

    uint16_t front_left = (motor_thrust + motor_roll + motor_pitch + motor_yaw) / 4;
    uint16_t front_right = (motor_thrust - motor_roll + motor_pitch - motor_yaw) / 4;
    uint16_t rear_left = (motor_thrust + motor_roll - motor_pitch - motor_yaw) / 4;
    uint16_t rear_right = (motor_thrust - motor_roll - motor_pitch + motor_yaw) / 4;

    DBG_PRINTVAR(2, front_left);
    DBG_PRINTVAR(2, front_right);
    DBG_PRINTVAR(2, rear_left);
    DBG_PRINTVAR(2, rear_right);

    if (front_left > SERVO_RANGE ||
        front_right > SERVO_RANGE ||
        rear_left > SERVO_RANGE ||
        rear_right > SERVO_RANGE)
    {
        halt(HALT_SERVO_INPUT_OUT_OF_RANGE);
        return;
    }

    fl.writeMicroseconds(SERVO_MIN + front_left);
    fr.writeMicroseconds(SERVO_MIN + front_right);
    rl.writeMicroseconds(SERVO_MIN + rear_left);
    rr.writeMicroseconds(SERVO_MIN + rear_right);
}
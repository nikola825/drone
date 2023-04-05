#include "common.h"
#include "motors.h"

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

constexpr int grounds[4]={FL_GND, FR_GND, RR_GND, RL_GND};

Servo fl;
Servo fr;
Servo rr;
Servo rl;

void init_motors()
{
    for(int i=0;i<4;i++)
    {
        digitalWrite(grounds[i], LOW);
        pinMode(grounds[i], OUTPUT);
        digitalWrite(grounds[i], LOW);
    }

    fl.attach(FL_PIN, 1000, 2000);
    fl.write(0);

    fr.attach(FR_PIN, 1000, 2000);
    fr.write(0);

    rr.attach(RR_PIN, 1000, 2000);
    rr.write(0);

    rl.attach(RL_PIN, 1000, 2000);
    rl.write(0);
}

void set_thrust(uint16_t thrust)
{
    thrust = 1000+thrust;
    fl.writeMicroseconds(thrust);
    fr.writeMicroseconds(thrust);
    rr.writeMicroseconds(thrust);
    rl.writeMicroseconds(thrust);
}

void motor_start_fancy()
{
    fl.write(FANCY_LOW);
    delay(500);
    fl.write(FANCY_HIGH);
    delay(1000);
    fr.write(FANCY_LOW);
    delay(500);
    fr.write(FANCY_HIGH);
    delay(1000);
    rr.write(FANCY_LOW);
    delay(500);
    rr.write(FANCY_HIGH);
    delay(1000);
    rl.write(FANCY_LOW);
    delay(500);
    rl.write(FANCY_HIGH);
    delay(1000);
}
//
// Created by nidzo on 8/5/23.
//
#include <I2Cdev.h>
#include <MPU6050_6Axis_MotionApps20.h>
#include <Wire.h>
#include "common.h"
#include <math.h>

MPU6050 mpu;
uint8_t fifoBuffer[64];

Quaternion q;           // [w, x, y, z]         quaternion container
VectorInt16 aa;         // [x, y, z]            accel sensor measurements
VectorInt16 aaReal;     // [x, y, z]            gravity-free accel sensor measurements
VectorInt16 aaWorld;    // [x, y, z]            world-frame accel sensor measurements
VectorFloat gravity;    // [x, y, z]            gravity vector
float euler[3];         // [psi, theta, phi]    Euler angle container
float ypr[3];           // [yaw, pitch, roll]   yaw/pitch/roll container and gravity vector

constexpr float RADIANS_TO_DEGREES_FACTOR = 180.0f * 0.31830988618379067154f;

void init_motion()
{
    Wire.begin();
    Wire.setClock(400000);
    Wire.setWireTimeout(100000, false);
    Wire.clearWireTimeoutFlag();

    DBG_PRINTLN(1, "Initializing MPU");
    mpu.initialize();

    DBG_PRINTLN(0, "Testing MPU connection");
    if (mpu.testConnection())
    {
        DBG_PRINTLN(0, "MPU connection OK");

        DBG_PRINTVAR(0, "Flashing DMP");
        uint8_t dmpInitStatus = mpu.dmpInitialize();
        if (dmpInitStatus != 0)
        {
            halt(HALT_DMP_FAILED);
        }
        else
        {
            DBG_PRINTVAR(0, "DMP flash OK");

            mpu.setDLPFMode(0);

            mpu.setXAccelOffset(-2630);
            mpu.setYAccelOffset(-1512);
            mpu.setZAccelOffset(1443);

            mpu.setXGyroOffset(5);
            mpu.setYGyroOffset(38);
            mpu.setZGyroOffset(58);

            mpu.setRate(0);

            // Calibration Time: generate offsets and calibrate our MPU6050
            mpu.CalibrateAccel(6);
            mpu.CalibrateGyro(6);
            mpu.PrintActiveOffsets();

            // turn on the DMP, now that it's ready
            DBG_PRINTLN(0, "Enabling DMP...");
            mpu.setDMPEnabled(true);
            mpu.getIntStatus();
        }
    }
    else
    {
        halt(HALT_MPU_FAILED);
    }
}

void raw_ypr(float &yaw, float &pitch, float &roll)
{
    static float yaw_offset=0, pitch_offset=0, roll_offset=0;
    static float yaw_prev=0, pitch_prev=0, roll_prev=0;
    static unsigned long prev_t = millis();
    unsigned long t = millis();
    unsigned long dt=t-prev_t;
    static unsigned long calib_count = 100;

    int iroll_rotation, ipitch_rotation, iyaw_rotation;
    mpu.getRotation(&iroll_rotation, &ipitch_rotation, &iyaw_rotation);

    float roll_rotation = iroll_rotation;
    float pitch_rotation = ipitch_rotation;
    float yaw_rotation = iyaw_rotation;

    if(calib_count)
    {
        yaw_offset += yaw_rotation/100.0f;
        pitch_offset += pitch_rotation/100.0f;
        roll_offset += roll_rotation/100.0f;

        calib_count--;

        yaw=pitch=roll=0;
    }
    else
    {
        yaw_rotation -= yaw_offset;
        pitch_rotation -= pitch_offset;
        roll_rotation -= roll_offset;
        yaw = yaw_prev + yaw_rotation/16.4f*dt/1000;
        pitch = pitch_prev + pitch_rotation/16.4f*dt/1000;
        roll = roll_prev + roll_rotation/16.4f*dt/1000;

        /*yaw=yaw_offset;
        pitch=pitch_offset;
        roll=roll_offset;*/
    }

    yaw_prev = yaw;
    roll_prev = roll;
    pitch_prev = pitch;
}

void get_ypr(float &y, float &p, float &r)
{
    //raw_ypr(y, p, r);
    while (mpu.dmpGetCurrentFIFOPacket(fifoBuffer))
    {
        mpu.dmpGetQuaternion(&q, fifoBuffer);
        mpu.dmpGetGravity(&gravity, &q);
        mpu.dmpGetYawPitchRoll(ypr, &q, &gravity);
    }
    if(Wire.getWireTimeoutFlag())
    {
        halt(MPU_TIMEOUT);
    }

    y = ypr[0] * RADIANS_TO_DEGREES_FACTOR;
    p = ypr[1] * RADIANS_TO_DEGREES_FACTOR;
    r = ypr[2] * RADIANS_TO_DEGREES_FACTOR;
}
//
// Created by nidzo on 8/5/23.
//
#include <I2Cdev.h>
#include <MPU6050_6Axis_MotionApps20.h>
#include <Wire.h>
#include "common.h"

MPU6050 mpu;
uint8_t fifoBuffer[64];

Quaternion q;           // [w, x, y, z]         quaternion container
VectorInt16 aa;         // [x, y, z]            accel sensor measurements
VectorInt16 aaReal;     // [x, y, z]            gravity-free accel sensor measurements
VectorInt16 aaWorld;    // [x, y, z]            world-frame accel sensor measurements
VectorFloat gravity;    // [x, y, z]            gravity vector
float euler[3];         // [psi, theta, phi]    Euler angle container
float ypr[3];           // [yaw, pitch, roll]   yaw/pitch/roll container and gravity vector

void init_motion()
{
    Wire.begin();
    Wire.setClock(400000);

    DBG_PRINTLN(1, "Initializing MPU");
    mpu.initialize();

    DBG_PRINTLN(0, "Testing MPU connection");
    if(mpu.testConnection())
    {
        DBG_PRINTLN(0, "MPU connection OK");

        DBG_PRINTVAR(0, "Flashing DMP");
        uint8_t  dmpInitStatus = mpu.dmpInitialize();
        if(dmpInitStatus != 0)
        {
            halt(HALT_DMP_FAILED);
        }
        else
        {
            DBG_PRINTVAR(0, "DMP flash OK");
            mpu.setXAccelOffset(-4757);
            mpu.setYAccelOffset(-106);
            mpu.setZAccelOffset(4648);

            mpu.setXGyroOffset(139);
            mpu.setYGyroOffset(-7);
            mpu.setZGyroOffset(-2);

            mpu.setRate(1);

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

void get_ypr(float &y, float &p, float &r)
{
    if (mpu.dmpGetCurrentFIFOPacket(fifoBuffer))
    {
        mpu.dmpGetQuaternion(&q, fifoBuffer);
        mpu.dmpGetGravity(&gravity, &q);
        mpu.dmpGetYawPitchRoll(ypr, &q, &gravity);

        y = ypr[0];
        p = ypr[1];
        r = ypr[2];
    }
}
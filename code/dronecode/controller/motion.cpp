//
// Created by nidzo on 8/5/23.
//
#include <Wire.h>
#include <MPU6050_6Axis_MotionApps20.h>
#include "common.h"
#include "bluetooth.h"
#include "motion.h"

MPU6050 mpu;

void get_ypr(float &yaw, float &pitch, float &roll)
{
    int16_t xr, yr, zr;

    if (!mpu.testConnection())
    {
        halt(HALT_ERROR::MPU_FAIL);
    }

    mpu.getRotation(&xr, &yr, &zr);
    constexpr float divisor = 131.0;

    pitch = xr / divisor;
    roll = yr / divisor;
    yaw = zr / divisor;
}

void init_motion()
{
    pinMode(16, OUTPUT);
    digitalWrite(16, LOW);
    pinMode(17, OUTPUT);
    digitalWrite(17, HIGH);
    delay(10);
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

        mpu.setFullScaleGyroRange(MPU6050_IMU::MPU6050_GYRO_FS_250);
        mpu.setFullScaleAccelRange(MPU6050_IMU::MPU6050_ACCEL_FS_4);
        mpu.setDLPFMode(MPU6050_IMU::DLPF_CFG::MPU6050_DLPF_BW_42);
        mpu.setRate(0); // 1KHz/(1+0) = 1KHz

        mpu.setXAccelOffset(-3343);
        mpu.setYAccelOffset(-951);
        mpu.setZAccelOffset(729);

        mpu.setXGyroOffset(41);
        mpu.setYGyroOffset(-72);
        mpu.setZGyroOffset(-28);/**/


        // Calibration Time: generate offsets and calibrate our MPU6050
        mpu.CalibrateAccel(6);
        mpu.CalibrateGyro(6);
        mpu.PrintActiveOffsets();

        // turn on the DMP, now that it's ready
        mpu.getIntStatus();
    }
    else
    {
        halt(HALT_ERROR::MPU_FAIL);
    }
}


void get_ypra(float &x, float &y, float &z, bool still)
{
    static float gx = 0, gy = 0, gz = 0;

    constexpr float divisor = 8192.0;
    constexpr float G = 9.80665;
    constexpr float coeff = G / divisor;
    int16_t xa, ya, za;
    mpu.getAcceleration(&xa, &ya, &za);

    x = xa * coeff;
    y = ya * coeff;
    z = za * coeff;

    if (still)
    {
        gx = gx * 0.99 + x * 0.01;
        gy = gy * 0.99 + y * 0.01;
        gz = gz * 0.99 + z * 0.01;
    }
    x -= gx;
    y -= gy;
    z -= gz;
}

void mpu_set_rate(uint8_t rate)
{
    if (rate <= 4)
    {
        DBG_PRINTLN(0, "Set rate")
        mpu.setRate(rate);
    }
}

void mpu_set_dlpf(uint8_t dlpf)
{
    if (dlpf <= 6)
    {
        DBG_PRINTLN(0, "Set dlpf")
        mpu.setDLPFMode(dlpf);
    }
}
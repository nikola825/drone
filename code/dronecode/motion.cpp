//
// Created by nidzo on 8/5/23.
//
#include <Wire.h>
#include <MPU6050_6Axis_MotionApps20.h>
#include "common.h"
#include "bluetooth.h"
#include "motion.h"

int16_t ypr_binary[3];
uint8_t calib_data_check[CALIB_BLOCK_LEN];
MPU6050 mpu;

constexpr float RADIANS_TO_DEGREES_FACTOR = 180.0f * 0.31830988618379067154f;
uint8_t fifoBuffer[64];
float euler[3];         // [psi, theta, phi]    Euler angle container
float ypr[3];           // [yaw, pitch, roll]   yaw/pitch/roll container and gravity vector
VectorFloat gravity;    // [x, y, z]            gravity vector
Quaternion q;

uint8_t read8(uint8_t address)
{
    uint8_t data;
    Wire.beginTransmission(MPU_ADDRESS);
    Wire.write(address);

    if(Wire.endTransmission(true) != 0)
    {
        halt(WIRE_FAIL);
    }

    if(Wire.requestFrom(MPU_ADDRESS, 1, true) != 1)
    {
        halt(WIRE_FAIL);
    }

    if(Wire.available())
    {
        data = Wire.read();
    }
    else
    {
        halt(WIRE_FAIL);
    }
    return data;
}

uint16_t read16(uint8_t address)
{
    uint16_t data;
    read_buffer(address, 2, &data);
    return data;
}

void read_buffer(uint8_t address, int len, void* buffer)
{
    Wire.beginTransmission(MPU_ADDRESS);
    if(Wire.getWireTimeoutFlag())
    {
        halt(WIRE_FAIL);
    }
    Wire.write(address);
    if(Wire.getWireTimeoutFlag())
    {
        halt(WIRE_FAIL);
    }

    if(Wire.endTransmission(true) != 0)
    {
        halt(WIRE_FAIL);
    }
    if(Wire.getWireTimeoutFlag())
    {
        halt(WIRE_FAIL);
    }

    if(Wire.requestFrom(MPU_ADDRESS, len, true) != len)
    {
        halt(WIRE_FAIL);
    }
    if(Wire.getWireTimeoutFlag())
    {
        halt(WIRE_FAIL);
    }

    if(Wire.readBytes((uint8_t*)buffer, len) != len)
    {
        halt(WIRE_FAIL);
    }
    if(Wire.getWireTimeoutFlag())
    {
        halt(WIRE_FAIL);
    }
}

void write8(uint8_t address, uint8_t data)
{
    Wire.beginTransmission(MPU_ADDRESS);
    Wire.write(address);

    Wire.write(data);

    if(Wire.endTransmission(true) != 0)
    {
        halt(WIRE_FAIL);
    }
}

void get_ypr(float &yaw, float &pitch, float &roll)
{
    int16_t xr, yr, zr;
    mpu.getRotation(&xr, &yr, &zr);

    float divisor = 131.0;

    pitch=xr/divisor;
    roll=-yr/divisor;
    yaw=-zr/divisor;
}

void init_mpu()
{
    DBG_PRINTLN(1,  "Motion init");
    delay(700);
    Wire.setClock(400000);
    Wire.begin();
    Wire.setTimeout(0);
    Wire.clearWireTimeoutFlag();

    DBG_PRINTLN(2, "Wire init done");

    if(read8(CHIP_ID) != 0xa0)
    {
        DBG_PRINTLN(1, "Invalid BNO chip id");
        halt(MPU_FAIL);
    }
    DBG_PRINTLN(2, "BNO chip id OK");

    if(read8(ACC_ID) != 0xfb)
    {
        DBG_PRINTLN(1, "Invalid BNO acc id");
        halt(MPU_FAIL);
    }
    DBG_PRINTLN(2, "BNO acc id OK");

    if(read8(MAG_ID) != 0x32)
    {
        DBG_PRINTLN(1, "Invalid BNO mag id");
        halt(MPU_FAIL);
    }
    DBG_PRINTLN(2, "BNO mag id OK");

    if(read8(GYR_ID) != 0x0f)
    {
        DBG_PRINTLN(1, "Invalid BNO gyro id");
        halt(MPU_FAIL);
    }
    DBG_PRINTLN(2, "BNO gyro id OK");

    DBG_PRINTLN(1, "BNO triggering reset");

    uint8_t trigger_value = read8(SYS_TRIGGER);
    trigger_value |= SYS_TRIGGER_RESET;

    write8(SYS_TRIGGER, trigger_value);

    delay(750);

    if((read8(ST_RESULT) & 0x0f) != 0x0f)
    {
        DBG_PRINTLN(1, "BNO POST fail");
        halt(MPU_FAIL);
    }
    DBG_PRINTLN(1, "BNO POST OK");
}

void mpu_setmode_config()
{
    write8(OPR_MODE, OPR_MODE_CONFIG);
    delay(750);
}

void mpu_setmode_fusion()
{
    write8(OPR_MODE, OPR_MODE_GYROONLY);
    delay(750);
}

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

        mpu.setFullScaleGyroRange(MPU6050_IMU::MPU6050_GYRO_FS_250);
        mpu.setDLPFMode(MPU6050_IMU::MPU6050_DLPF_BW_10);

        /*mpu.setXAccelOffset(-2630);
        mpu.setYAccelOffset(-1512);
        mpu.setZAccelOffset(1443);

        mpu.setXGyroOffset(5);
        mpu.setYGyroOffset(38);
        mpu.setZGyroOffset(58);/**/
        mpu.setXAccelOffset(-4757);
        mpu.setYAccelOffset(-106);
        mpu.setZAccelOffset(4648);

        mpu.setXGyroOffset(139);
        mpu.setYGyroOffset(-7);
        mpu.setZGyroOffset(-2);/**/

        mpu.setRate(4);

        // Calibration Time: generate offsets and calibrate our MPU6050
        mpu.CalibrateAccel(6);
        mpu.CalibrateGyro(6);
        mpu.PrintActiveOffsets();

        // turn on the DMP, now that it's ready
        mpu.getIntStatus();
    }
    else
    {
        halt(MPU_FAIL);
    }
}


void get_ypra(float &y, float &p, float &r)
{
    //raw_ypr(y, p, r);
    /*while (mpu.dmpGetCurrentFIFOPacket(fifoBuffer))
    {
        mpu.dmpGetQuaternion(&q, fifoBuffer);
        mpu.dmpGetGravity(&gravity, &q);
        mpu.dmpGetYawPitchRoll(ypr, &q, &gravity);
    }
    if(Wire.getWireTimeoutFlag())
    {
        halt(MPU_FAIL);
    }*/

    y = ypr[0] * RADIANS_TO_DEGREES_FACTOR;
    p = ypr[1] * RADIANS_TO_DEGREES_FACTOR;
    r = ypr[2] * RADIANS_TO_DEGREES_FACTOR;
}
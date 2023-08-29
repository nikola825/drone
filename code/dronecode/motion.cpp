//
// Created by nidzo on 8/5/23.
//
#include <Wire.h>
#include "common.h"
#include "bluetooth.h"
#include "motion.h"

int16_t ypr_binary[3];
uint8_t calib_data_check[CALIB_BLOCK_LEN];

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

    read_buffer(GYR_DATA_X_LSB, 6, ypr_binary);
    if(Wire.getWireTimeoutFlag())
    {
        halt(WIRE_FAIL);
    }

    yaw = ypr_binary[2]/16.0;
    pitch = ypr_binary[1]/16.0;
    roll = ypr_binary[0]/16.0;

    /*read_buffer(EUL_Heading_LSB, 6, ypr_binary);

    yaw = ypr_binary[0]/16.0;
    pitch = ypr_binary[1]/16.0;
    roll = ypr_binary[2]/16.0;*/
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
    init_mpu();

    load_calib_data();

    DBG_PRINTLN(1, "Writing calib data");
    for(int i=0;i<CALIB_BLOCK_LEN; i++)
    {
        write8(CALIB_BLOCK_START+i, calib_data[i]);
    }


    DBG_PRINTLN(2, "Verifying calib data");
    read_buffer(CALIB_BLOCK_START, CALIB_BLOCK_LEN, calib_data_check);
    for(int i=0;i<CALIB_BLOCK_LEN; i++)
    {
        if(calib_data_check[i]!=calib_data[i])
        {
            DBG_PRINTLN(1, "Calib verify fail");
            halt(MPU_FAIL);
        }
    }

    DBG_PRINTLN(1, "Calib write ok");

    mpu_setmode_fusion();

    if(read8(SYS_STATUS) != 6 || read8(SYS_ERR) != 0)
    {
        halt(MPU_FAIL);
    }
}
//
// Created by nidzo on 8/5/23.
//

#ifndef CODE_MOTION_H
#define CODE_MOTION_H

constexpr int MPU_ADDRESS = 0x28;

constexpr uint8_t CHIP_ID = 0x00;
constexpr uint8_t ACC_ID = 0x01;
constexpr uint8_t MAG_ID = 0x02;
constexpr uint8_t GYR_ID = 0x03;
constexpr uint8_t ST_RESULT = 0x36;
constexpr uint8_t CALIB_STAT = 0x35;
constexpr uint8_t SYS_STATUS = 0x39;
constexpr uint8_t SYS_ERR = 0x3a;
constexpr uint8_t OPR_MODE = 0x3d;
constexpr uint8_t SYS_TRIGGER = 0x3f;
constexpr uint8_t EUL_Heading_LSB = 0x1a;
constexpr uint8_t EUL_Roll_LSB = 0x1c;
constexpr uint8_t EUL_Pitch_LSB = 0x1e;
constexpr uint8_t GYR_DATA_X_LSB = 0x14;

constexpr uint8_t OPR_MODE_NDOF = 0x0c;
constexpr uint8_t OPR_MODE_CONFIG = 0x00;
constexpr uint8_t OPR_MODE_GYROONLY = 0x03;
constexpr uint8_t SYS_TRIGGER_RESET = 0x20;

constexpr uint8_t CALIB_BLOCK_START = 0x55;
constexpr uint8_t CALIB_BLOCK_LEN = 22;

void init_motion();
void get_ypr(float &y, float &p, float &r);
void get_ypra(float &y, float &p, float &r, bool still);

uint8_t read8(uint8_t address);
uint16_t read16(uint16_t address);
void read_buffer(uint8_t address, int len, void* buffer);
void write8(uint8_t address, uint8_t data);
void init_mpu();
void mpu_setmode_config();
void mpu_setmode_fusion();

void store_calib_data();
void load_calib_data();

extern uint8_t calib_data[CALIB_BLOCK_LEN];
#endif //CODE_MOTION_H
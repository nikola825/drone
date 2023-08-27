#include <EEPROM.h>
#include "common.h"
#include "bluetooth.h"
#include "motion.h"

constexpr uint8_t CALIB_EEPROM_START = 0x02;
uint8_t calib_data[CALIB_BLOCK_LEN];

void store_calib_data()
{
    uint8_t checksum = 0;
    for (int i = 0; i < CALIB_BLOCK_LEN; ++i)
    {
        EEPROM.update(CALIB_EEPROM_START + i, calib_data[i]);
        checksum ^= calib_data[i];
    }
    EEPROM.update(CALIB_EEPROM_START + CALIB_BLOCK_LEN, checksum);
    delay(5*CALIB_BLOCK_LEN);
}

void load_calib_data()
{
    uint8_t checksum = 0;
    for (int i = 0; i < CALIB_BLOCK_LEN; ++i)
    {
        calib_data[i] = EEPROM.read(CALIB_EEPROM_START + i);
        checksum ^= calib_data[i];
    }
    if(checksum != EEPROM.read(CALIB_EEPROM_START + CALIB_BLOCK_LEN))
    {
        halt(EEPROM_FAIL);
    }
}

void calibrate_mpu()
{
    Serial.begin(9600);
    bt_init();

    init_mpu();

    mpu_setmode_fusion();

    uint8_t calib_ok = 0;

    while (1)
    {
        delay(500);
        if (calib_ok)
        {
            uint8_t calib_stat = read8(CALIB_STAT);
            bt_send_int(1, calib_stat);

            read_buffer(CALIB_BLOCK_START, CALIB_BLOCK_LEN, calib_data);
            for (int i = 0; i < 22; i++)
            {
                bt_send_int(i + 2, calib_data[i]);
            }
            store_calib_data();
        }
        else
        {
            uint8_t calib_stat = read8(CALIB_STAT);
            bt_send_int(1, calib_stat);
            float y, p, r;
            get_ypr(y, p, r);
            bt_send_float(25, y);
            bt_send_float(26, p);
            bt_send_float(27, r);

            if (calib_stat == 0xff)
            {
                calib_ok = 1;
                mpu_setmode_config();
            }
        }
    }

}

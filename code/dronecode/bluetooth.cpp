#include "bluetooth.h"

constexpr uint32_t BT_COMM_BAUD_RATE = 115200;
constexpr uint32_t BT_AT_BAUD_RATE = 38400;
constexpr uint32_t SERIAL_AT_BAUD_RATE = 9600;

void bt_init()
{
    bluetooth_port.begin(BT_COMM_BAUD_RATE);
}

#ifdef BT_DEBUG

void bt_send_int(uint8_t id, uint32_t value)
{
    uint8_t *cursor = (uint8_t*)(&value);
    bluetooth_port.write((uint8_t) 0x42);
    bluetooth_port.write((uint8_t)0x00);
    bluetooth_port.write((uint8_t)id);
    for(int i=0;i<sizeof(value);++i)
    {
        bluetooth_port.write(*cursor);
        ++cursor;
    }
    bluetooth_port.write((uint8_t)0x24);
}

void bt_send_float(uint8_t id, float value)
{
    uint8_t *cursor = (uint8_t*)(&value);
    bluetooth_port.write((uint8_t)0x42);
    bluetooth_port.write((uint8_t)0x1);
    bluetooth_port.write((uint8_t)id);
    int i=0;
    for(;i<sizeof(value);++i)
    {
        bluetooth_port.write(*cursor);
        ++cursor;
    }
    bluetooth_port.write((uint8_t)0x24);
}

#else

#define bt_send_int(uint8_t, uint16_t)
#define bt_send_float(uint8_t, uint16_t)

#endif

/*
AT
AT+NAME:DrnTest
AT+PSWD:1234
AT+UART:57600,0,0
AT+ROLE:0
*/
void bt_at_terminal()
{
    Serial.begin(SERIAL_AT_BAUD_RATE);
    bluetooth_port.begin(BT_AT_BAUD_RATE);
    bluetooth_port.setTimeout(1000000);
    Serial.setTimeout(1000000);

    Serial.println("Bluetooth AT terminal:");
    while(1)
    {

        while(Serial.available())
        {
            char c=Serial.read();
            if(c==10)
            {
                bluetooth_port.print("\r\n");
                break;
            }
            else
            {
                bluetooth_port.write(c);
            }
        }
        delay(100);
        while(bluetooth_port.available())
        {
            char c=bluetooth_port.read();
            
            Serial.write(c);
        }
    }
}
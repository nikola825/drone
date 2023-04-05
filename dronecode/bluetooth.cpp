#include "bluetooth.h"

constexpr uint32_t BT_COMM_BAUD_RATE = 115200;
constexpr uint32_t BT_AT_BAUD_RATE = 38400;
constexpr uint32_t SERIAL_AT_BAUD_RATE = 9600;

HardwareSerial &bluetooth_port;

void bt_init()
{
    bluetooth_port.begin(BT_COMM_BAUD_RATE);
}

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
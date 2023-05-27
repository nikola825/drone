#include "common.h"
#include "storage.h"
#include "bluetooth.h"
#include "motors.h"
#include "commands.h"
#include "navigation.h"

bool halted=false;
uint8_t global_error;

constexpr int ERROR_FLASH_PERIOD = 300;
constexpr int ERROR_FLASH_DELAY = 1000;

void flash_error()
{
    for(int i=0; i<global_error; i++)
    {
        digitalWrite(LED_BUILTIN, HIGH);
        delay(ERROR_FLASH_PERIOD);
        digitalWrite(LED_BUILTIN, LOW);
        delay(ERROR_FLASH_PERIOD);
    }
    delay(ERROR_FLASH_DELAY);
}

void setup()
{
    bt_init();
    init_commands();
    init_navigation();
    init_motors();
    pinMode(LED_BUILTIN, OUTPUT);
    Serial.begin(9600);

    digitalWrite(LED_BUILTIN, HIGH);

    DBG_PRINTLN(1, "Init done");
    delay(100);
}

void loop()
{
    DBG_PRINTLN(2, "Loop");
    process_commands(bluetooth_port);
    navigate();
    drive();
}

void halt(uint8_t error)
{
    DBG_PRINTLN(1, "Halting");
    DBG_PRINTVAR(1, error);
    halted=true;
    stop_motors();
    global_error = error;
    while(1)
    {
        flash_error();
    }
}
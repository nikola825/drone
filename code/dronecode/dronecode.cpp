#include "common.h"
#include "storage.h"
#include "bluetooth.h"
#include "motors.h"
#include "commands.h"
#include "navigation.h"
#include "motion.h"
#include "mpucalib.h"


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
    pinMode(LED_BUILTIN, OUTPUT);
    digitalWrite(LED_BUILTIN, LOW);

    //calibrate_mpu();
    Serial.begin(9600);
    bt_init();
    init_storage();
    init_commands();
    init_navigation();
    init_motors();
    init_motion();

    digitalWrite(LED_BUILTIN, HIGH);

    DBG_PRINTLN(1, "Init done");
    delay(100);
}

void loop()
{
    unsigned long x= millis();
    DBG_PRINTLN(2, "Loop");
    digitalWrite(LED_BUILTIN, LOW);
    process_commands(bluetooth_port);
    navigate();
    drive();
    digitalWrite(LED_BUILTIN, HIGH);
    delay(10);
    //Serial.println(millis()-x);
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
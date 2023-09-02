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
bool watchdog_enabled = false;

constexpr int ERROR_FLASH_PERIOD = 300;
constexpr int ERROR_FLASH_DELAY = 1000;
void flash_error()
{
    wdt_reset();
    for(int i=0; i<global_error; i++)
    {
        wdt_reset();
        digitalWrite(LED_BUILTIN, HIGH);
        wdt_reset();
        delay(ERROR_FLASH_PERIOD/2);
        wdt_reset();
        delay(ERROR_FLASH_PERIOD/2);
        wdt_reset();
        digitalWrite(LED_BUILTIN, LOW);
        wdt_reset();
        delay(ERROR_FLASH_PERIOD/2);
        wdt_reset();
        delay(ERROR_FLASH_PERIOD/2);
        wdt_reset();
    }
    for(int i=0;i<10;i++)
    {
        wdt_reset();
        delay(ERROR_FLASH_DELAY/10);
    }
    wdt_reset();
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
    while (1)
    {
        wdt_reset();
        flash_error();
    }
}
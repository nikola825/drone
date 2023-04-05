#include"common.h"
#include"storage.h"
#include"bluetooth.h"
#include"motors.h"
#include"commands.h"

bool halted=false;

void setup()
{
    init_motors();
    setup_commands();
    pinMode(LED_BUILTIN, OUTPUT);
    Serial.begin(9600);
}

void loop()
{
    process_commands(bluetooth_port);    
}
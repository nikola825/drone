//
// Created by nidzo on 6/10/24.
//
#include "common.h"
#include <stdlib.h>
#include "motion.h"
#include "mpucalib.h"
#include "bluetooth.h"
#include "motors.h"
#include "commands.h"
#include "storage.h"
#include "navigation.h"

MAKE_PIN(BLUE, H, 2, 1);
MAKE_PIN(GREEN, H, 3, 1);
MAKE_PIN(LifeIndicator, B, 0, 1);

bool halted;
bool watchdog_enabled;

void setup()
{
    DDRH = 0;
    halted = false;
    DDR(H) |= 0x0c;
    DDRB|=1;
    DDRD&=251;
    PORTD|=4;

    pullup_BLUE();
    pullup_GREEN();
    pulldown_BLUE();
    pulldown_GREEN();
    watchdog_enabled = false;
    Serial.begin(115200);
    Serial.setTimeout(50);

    bt_init();
    init_commands();
    init_storage();
    init_navigation();
    init_motion();
    pulldown_GREEN();
    wdt_enable(WATCHDOG_TIMEOUT);
    init_motors();
    pulldown_BLUE();
}

void pullup_ERROR()
{
    pullup_BLUE();
}

void pulldown_ERROR()
{
    pulldown_BLUE();
}

void loop()
{
    pullup_LifeIndicator();
    process_commands(bluetooth_port);
    navigate();
    drive_motors();
    pulldown_LifeIndicator();
}
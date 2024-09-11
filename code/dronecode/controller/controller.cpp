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

MAKE_PIN(RED, C, 1, 1);

MAKE_PIN(YELLOW, C, 0, 1);

MAKE_PIN(GREEN, G, 1, 1);

MAKE_PIN(BLUE, G, 0, 1);

MAKE_PIN(LifeIndicator, B, 0, 1);

void setup()
{
    DDRC |= 3;
    DDRG |= 3;
    DDRB |= 1;

    pulldown_BLUE();
    pulldown_GREEN();
    pulldown_YELLOW();
    pulldown_RED();
    Serial.begin(115200);
    Serial.setTimeout(50);

    pullup_RED();
    bt_init();
    init_commands();
    init_storage();
    init_navigation();
    pullup_BLUE();
    init_motion();
    pullup_GREEN();
    wdt_enable(WATCHDOG_TIMEOUT);
    pullup_YELLOW();
    init_motors();
    pulldown_BLUE();
    pulldown_GREEN();
    pullup_RED();
    pullup_YELLOW();
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
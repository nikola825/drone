#include "common.h"
#include "navigation.h"
#include "storage.h"

void init_navigation()
{
    thrust_input = 0;
    yaw_input = 0;
    pitch_input = 0;
    roll_input = 0;
}

void navigate()
{
    DBG_PRINTVAR(2, thrust_input);
    DBG_PRINTVAR(2, yaw_input);
    DBG_PRINTVAR(2, pitch_input);
    DBG_PRINTVAR(2, roll_input);

    motor_thrust = thrust_input;
    motor_yaw = yaw_input;
    motor_pitch = pitch_input;
    motor_roll = roll_input;
}
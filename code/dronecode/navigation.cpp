#include "common.h"
#include "navigation.h"
#include "storage.h"
#include "motion.h"

#define ISGN(X) ((X)>=0?1:-1)
#define IABS(X) ((X)>=0?(X):(-(X)))
#define IMIN(X,Y) ((X)>(Y)?(Y):(X))
#define INPUT_LIMIT(INP, LIM) IMIN((LIM), IABS(INP))* ISGN(INP)

void init_navigation()
{
    thrust_input = 0;
    yaw_input = 0;
    pitch_input = 0;
    roll_input = 0;
}

void navigate()
{
    float yaw_measured, pitch_measured, roll_measured;

    get_ypr(yaw_measured, pitch_measured, roll_measured);

    DBG_PRINTVAR(2, thrust_input);
    DBG_PRINTVAR(2, yaw_input);
    DBG_PRINTVAR(2, pitch_input);
    DBG_PRINTVAR(2, roll_input);

    motor_thrust = thrust_input;

    uint16_t command_limit = thrust_input >> 2;

    motor_yaw = INPUT_LIMIT(yaw_input, command_limit);
    motor_pitch = INPUT_LIMIT(pitch_input, command_limit);
    motor_roll = INPUT_LIMIT(roll_input, command_limit);
}
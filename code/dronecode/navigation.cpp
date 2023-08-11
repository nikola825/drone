#include "common.h"
#include "navigation.h"
#include "storage.h"
#include "motion.h"
#include "bluetooth.h"

#define ISGN(X) ((X)>=0?1:-1)
#define IABS(X) ((X)>=0?(X):(-(X)))
#define IMIN(X, Y) ((X)>(Y)?(Y):(X))
#define RANGE_LIMIT(INP, LIM) IMIN((LIM), IABS(INP))* ISGN(INP)
#define THRUST_NAVIGATION_THRESHOLD 1000

struct PidState
{
    float lasterror;
    float i;
    float target;

    void reset(float target_)
    {
        lasterror = 0;
        i = 0;
        this->target = target_;
    }
};

PidState yaw_pid, roll_pid, pitch_pid;

void reset_pids(float yaw = 0, float pitch = 0, float roll = 0)
{
    yaw_pid.reset(yaw);
    pitch_pid.reset(0);
    roll_pid.reset(0);
}

void init_navigation()
{
    thrust_input = 0;
    yaw_input = 0;
    pitch_input = 0;
    roll_input = 0;
    reset_pids();
}

void pid(const float e0, const float e1, const uint16_t dt, const float kp, const float ki, const float kd,
         const float limit, float &i,
         float &o)
{
    i += (e1 + e0) / 2 * dt * ki;
    i = RANGE_LIMIT(i, limit);


    float d = (e1 - e0) / dt * kd;

    float p = kp * e1;
    o = p + i + d;
    o = RANGE_LIMIT(o, limit);

}

float fit_angle(float angle)
{
    while (angle < -180)
    {
        angle += 360;
    }
    while (angle > 180)
    {
        angle -= 360;
    }
    return angle;
}

float angle_error(float target, float value)
{
    float f1 = target - value;
    float f2;
    if (f1 > 0)
    {
        f2 = f1 - 360;
    }
    else
    {
        f2 = target + 360;
    }
    if (fabs(f1) < fabs(f2))
    {
        return f1;
    }
    else
    {
        return f2;
    }
}

void navigate()
{
    static unsigned long past_time = millis();
    unsigned long current_time = millis();

    unsigned long dt = current_time - past_time;
    past_time = current_time;

    float yaw_measured, pitch_measured, roll_measured;

    get_ypr(yaw_measured, pitch_measured, roll_measured);

    if (thrust_input < THRUST_NAVIGATION_THRESHOLD)
    {
        reset_pids(fit_angle(yaw_measured - yaw_input), 0, 0);
    }


    DBG_PRINTVAR(2, thrust_input);
    DBG_PRINTVAR(2, yaw_input);
    DBG_PRINTVAR(2, pitch_input);
    DBG_PRINTVAR(2, roll_input);

    motor_thrust = thrust_input;

    uint16_t command_limit = thrust_input >> 2;

    float yaw_error = -angle_error(fit_angle(yaw_pid.target + yaw_input), yaw_measured);
    float yaw_pid_result;
    pid(yaw_pid.lasterror, yaw_error, dt, yaw_kp, 0.001f * yaw_ki, 0.01f * yaw_kd, command_limit,
        yaw_pid.i, yaw_pid_result);
    yaw_pid.lasterror = yaw_error;

    float roll_error = -angle_error(roll_pid.target, roll_measured);
    float roll_pid_result;
    pid(roll_pid.lasterror, roll_error, dt, roll_kp, 0.001*roll_ki, 0.01f*roll_kd, 100, roll_pid.i, roll_pid_result);
    roll_pid.lasterror = roll_error;

    float pitch_error = -angle_error(pitch_pid.target, pitch_measured);
    float pitch_pid_result;
    pid(pitch_pid.lasterror, pitch_error, dt, pitch_kp, 0.001*pitch_ki, 0.01f*pitch_kd, 100, pitch_pid.i, pitch_pid_result);
    pitch_pid.lasterror = pitch_error;

    bt_send_float(1, yaw_error);
    bt_send_float(2, yaw_pid_result);

    bt_send_float(3, pitch_error);
    bt_send_float(4, pitch_pid_result);

    bt_send_float(5, roll_error);
    bt_send_float(6, roll_pid_result);

    motor_yaw = RANGE_LIMIT(yaw_pid_result, command_limit);
    motor_pitch = RANGE_LIMIT(pitch_pid_result, command_limit);
    motor_roll = RANGE_LIMIT(roll_pid_result, command_limit);
}
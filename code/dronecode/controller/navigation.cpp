#include "common.h"
#include "navigation.h"
#include "storage.h"
#include "motion.h"
#include "bluetooth.h"

#define ISGN(X) ((X)>=0?1:-1)
#define IABS(X) ((X)>=0?(X):(-(X)))
#define IMIN(X, Y) ((X)>(Y)?(Y):(X))
#define RANGE_LIMIT(INP, LIM) IMIN((LIM), IABS(INP))* ISGN(INP)
#define THRUST_NAVIGATION_THRESHOLD 900

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

void pid(const float e0,
         const float e1,
         const float dt,
         const float kp,
         const float ki,
         const float kd,
         const float limit,
         float &i,
         float &o)
{
    i += e1 * dt * ki;
    i = RANGE_LIMIT(i, limit);


    float d = (e1 - e0) / dt * kd;

    float p = kp * e1;
    o = p + i + d;
    o = RANGE_LIMIT(o, limit);
}

float fit_angle(float angle)
{
    if (angle > 180)
    {
        return -(360 - angle);
    }
    return angle;
}

float angle_error(float target, float value)
{
    return target - value;
}

uint32_t prev_t = 0;
int x;

void push_float(float q)
{
    x = (x + 1) & 7;
    if (x == 0)
    {
        bt_send_float(1, q);
    }
}

uint16_t sensordump_ptr = 0;
float sensordump[1000];
bool flush_sensor_dump_next = false;

void flush_sensordump()
{
    if (!motors_enabled)
    {
        flush_sensor_dump_next = true;
    }
}

void do_flush_sensordump()
{
    if (!motors_enabled)
    {
        if (flush_sensor_dump_next)
        {
            for (uint16_t i = 0; i < 1000; i++)
            {
                bt_send_float(1, sensordump[i]);
                wdt_reset();
                delay(2);
            }
        }
        flush_sensor_dump_next = false;
    }
    else
    {
        flush_sensor_dump_next = false;
    }
}

void start_sensorlog()
{
    sensordump_ptr = 0;
    sensordump[999] = 0;
}

void push_point(float q, float dt)
{
    if (sensordump_ptr < 999)
    {
        sensordump[sensordump_ptr] = q;
        sensordump_ptr += 1;
        sensordump[999] += dt / 999;
    }
}

void navigate()
{
    constexpr float pid_scale_factor = 0.01;
    constexpr float input_scale_factor = 0.1;
    const int16_t command_limit = thrust_input >> 2;
    float yaw_measured, pitch_measured, roll_measured;
    get_ypr(yaw_measured, pitch_measured, roll_measured);
    uint32_t t = micros();
    uint32_t delta_t;
    if (t > prev_t)
    {
        delta_t = t - prev_t;
    }
    else
    {
        delta_t = (4294967295 - prev_t) + t;
    }
    prev_t = t;
    float dt = delta_t * 0.000001f;
    if (motors_enabled)
    {
        push_point(pitch_measured, dt);
    }
    else
    {
        do_flush_sensordump();
    }


    motor_thrust = thrust_input;
    motor_pitch = 0;
    motor_roll = 0;
    motor_yaw = 0;
    //if(motors_enabled) push_float(pitch_measured);
    if (thrust_input < THRUST_NAVIGATION_THRESHOLD)
    {
        reset_pids(0, 0, 0);
        motor_yaw = RANGE_LIMIT(0, command_limit);
        motor_pitch = RANGE_LIMIT(0, command_limit);
        motor_roll = RANGE_LIMIT(0, command_limit);
    }
    else
    {
        float yaw_error = yaw_pid.target + yaw_input - yaw_measured;
        float yaw_pid_result;
        pid(yaw_pid.lasterror,
            yaw_error,
            dt,
            yaw_kp * pid_scale_factor,
            yaw_ki * pid_scale_factor,
            yaw_kd * pid_scale_factor,
            command_limit,
            yaw_pid.i,
            yaw_pid_result);
        yaw_pid.lasterror = yaw_error;

        float pitch_error = pitch_pid.target - pitch_measured + pitch_input;
        float pitch_pid_result;
        pid(pitch_pid.lasterror,
            pitch_error,
            dt,
            pitch_kp * pid_scale_factor,
            pitch_ki * pid_scale_factor,
            pitch_kd * pid_scale_factor,
            command_limit,
            pitch_pid.i,
            pitch_pid_result);
        pitch_pid.lasterror = pitch_error;

        float roll_error = roll_pid.target + -roll_measured + roll_input;
        float roll_pid_result;
        pid(roll_pid.lasterror,
            roll_error,
            dt,
            roll_kp * pid_scale_factor,
            roll_ki * pid_scale_factor,
            roll_kd * pid_scale_factor,
            command_limit,
            roll_pid.i,
            roll_pid_result);
        roll_pid.lasterror = roll_error;

        motor_yaw = RANGE_LIMIT(yaw_pid_result, command_limit);
        motor_pitch = RANGE_LIMIT(pitch_pid_result, command_limit);
        motor_roll = RANGE_LIMIT(roll_pid_result, command_limit);
    }
}
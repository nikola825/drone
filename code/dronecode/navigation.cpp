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
    float output;

    void reset()
    {
        lasterror = 0;
        i = 0;
        output = 0;
    }
};

PidState yaw_velocity_pid, roll_velocity_pid, pitch_velocity_pid, roll_angle_pid, pitch_angle_pid;

void reset_pids(float yaw = 0, float pitch = 0, float roll = 0)
{
    yaw_velocity_pid.reset();
    pitch_velocity_pid.reset();
    roll_velocity_pid.reset();
    roll_angle_pid.reset();
    pitch_angle_pid.reset();
}

void init_navigation()
{
    thrust_input = 0;
    yaw_input = 0;
    pitch_input = 0;
    roll_input = 0;
    reset_pids();
}

void pid(
        PidState &pid,
        const float e,
        const float dt,
        const float kp,
        const float ki,
        const float kd,
        const float limit)
{
    constexpr float pid_scale_factor = 0.01;

    pid.i += e * dt * (ki * pid_scale_factor);
    pid.i = RANGE_LIMIT(pid.i, limit);


    float d = (e - pid.lasterror) / dt * (kd * pid_scale_factor);

    float p = (kp * pid_scale_factor) * e;
    pid.output = p + pid.i + d;
    pid.output = RANGE_LIMIT(pid.output, limit);
    pid.lasterror = e;
}

void navigate()
{
    static bool standing_still = true;
    motor_thrust = thrust_input;
    uint16_t command_limit = thrust_input >> 2;
    if (thrust_input > 0)
    {
        standing_still = false;
    }

    static unsigned long past_time = millis();
    unsigned long current_time = millis();

    float dt = (current_time - past_time) * 0.001;
    past_time = current_time;

    constexpr float input_scale_factor = 0.1;

    float yaw_velocity_measured, pitch_velocity_measured, roll_velocity_measured;
    get_ypr_velocities(yaw_velocity_measured, pitch_velocity_measured, roll_velocity_measured);

    float yaw_angle_measured, pitch_angle_measured, roll_angle_measured;
    get_ypr_angles(yaw_angle_measured, pitch_angle_measured, roll_angle_measured, standing_still);
    bt_send_float(1, yaw_angle_measured);
    bt_send_float(2, pitch_angle_measured);
    bt_send_float(3, roll_angle_measured);
    bt_send_float(4, yaw_velocity_measured);
    bt_send_float(5, pitch_velocity_measured);
    bt_send_float(6, roll_velocity_measured);

    if (thrust_input < THRUST_NAVIGATION_THRESHOLD)
    {
        reset_pids(0, 0, 0);
        motor_yaw = RANGE_LIMIT(0, command_limit);
        motor_pitch = RANGE_LIMIT(0, command_limit);
        motor_roll = RANGE_LIMIT(0, command_limit);
    }
    else
    {
        float roll_angle_error = roll_input * input_scale_factor - roll_angle_measured;
        pid(
                roll_angle_pid,
                roll_angle_error,
                dt,
                roll_angle_kp,
                roll_angle_ki,
                roll_angle_kd,
                40);

        float pitch_angle_error = pitch_input * input_scale_factor - pitch_angle_measured;
        pid(
                pitch_angle_pid,
                pitch_angle_error,
                dt,
                pitch_angle_kp,
                pitch_angle_ki,
                pitch_angle_kd,
                40);

        float yaw_velocity_error = yaw_input - yaw_velocity_measured;
        pid(
                yaw_velocity_pid,
                yaw_velocity_error,
                dt,
                yaw_velocity_kp,
                yaw_velocity_ki,
                yaw_velocity_kd,
                command_limit);


        float pitch_velocity_error = pitch_angle_pid.output - pitch_velocity_measured;
        pid(
                pitch_velocity_pid,
                pitch_velocity_error,
                dt,
                pitch_velocity_kp,
                pitch_velocity_ki,
                pitch_velocity_kd,
                command_limit);

        float roll_velocity_error = roll_angle_pid.output - roll_velocity_measured;
        pid(
                roll_velocity_pid,
                roll_velocity_error,
                dt,
                roll_velocity_kp,
                roll_velocity_ki,
                roll_velocity_kd,
                command_limit);


        motor_yaw = RANGE_LIMIT(yaw_velocity_pid.output, command_limit);
        motor_pitch = RANGE_LIMIT(pitch_velocity_pid.output, command_limit);
        motor_roll = RANGE_LIMIT(roll_velocity_pid.output, command_limit);
    }
}
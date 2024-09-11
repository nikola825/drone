//
// Created by nidzo on 8/5/23.
//

#ifndef CODE_MOTION_H
#define CODE_MOTION_H

void init_motion();

void get_ypr(float &y, float &p, float &r);

void get_ypra(float &y, float &p, float &r, bool still);

void mpu_set_rate(uint8_t rate);

void mpu_set_dlpf(uint8_t dlpf);

#endif //CODE_MOTION_H
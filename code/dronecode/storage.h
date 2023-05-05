#pragma once
#include "common.h"

uint8_t storage[1024];

#define STOREDVAR(TYPE,NAME,PREVIOUS) \
    TYPE &NAME=(TYPE&)(*(&PREVIOUS+1)); \
    

uint32_t storage_start=(float&)(storage[0]);
STOREDVAR(uint8_t, somevar, storage_start);
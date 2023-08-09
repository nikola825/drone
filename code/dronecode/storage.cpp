#include "common.h"

#define STOREDVAR(TYPE, NAME, PREVIOUS) \
    TYPE &NAME=(TYPE&)(*((&PREVIOUS)+1)); \

uint8_t storage[1024];

#include "storage.h"

uint32_t &storage_start = (uint32_t &) (storage[0]);

void init_storage()
{
    for (int i = 0; i < 1024; i++)
    {
        storage[i] = 0;
    }
}
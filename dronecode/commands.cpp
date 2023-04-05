#include "commands.h"

constexpr size_t BUFFER_SIZE = 256;
constexpr size_t COMMAND_SIZE=6;

uint8_t buffer[BUFFER_SIZE];

size_t buffer_start=0;
size_t buffer_end=0;

void (*(commands[256]))(uint8_t, uint8_t,uint8_t);

#define ADD_COMMAND(x)\
    commands[index++]=x;

void buffer_put(uint8_t data)
{
    buffer[buffer_end]=data;
    buffer_end=(buffer_end+1)%BUFFER_SIZE;
    if(buffer_end==buffer_start)
    {
        halt(HALT_BUFFER_OVERFLOW);
        return;
    }
}

uint8_t buffer_pop()
{
    if(buffer_start==buffer_end)
    {
        halt(HALT_BUFFER_UNDERFLOW);
        return;
    }
}

size_t buffer_available()
{
    if(buffer_end>=buffer_start)
    {
        return buffer_end-buffer_start;
    }
    else
    {
        return BUFFER_SIZE-buffer_start + buffer_end;
    }
}

void process_command()
{

}

void process_commands(HardwareSerial &serial)
{
    while(serial.available())
    {
        buffer_put(serial.read());
    }
    while(buffer_available()>=COMMAND_SIZE)
    {
        process_command();
    }
}

void halt(int error)
{
    halted=true;
}

void halt_command(uint8_t ignored0, uint8_t ignored1, uint8_t ignored2)
{

}

void start_command(uint8_t fancy, uint8_t ignored1, uint8_t ignored2)
{

}

void stop_command(uint8_t ignored0, uint8_t ignored1, uint8_t ignored2)
{
    
}

void setup_commands()
{
    size_t index=0;
    ADD_COMMAND(halt_command);
}
#include "commands.h"
//#include "motors.h"
#include "bluetooth.h"

#define COMMAND_SETUP_START
#define COMMAND_SETUP_END

constexpr size_t BUFFER_SIZE = 256;
constexpr size_t COMMAND_SIZE = 12;

constexpr uint8_t COMMAND_START_BYTE = 0x42;
constexpr uint8_t COMMAND_END_BYTE = 0x24;

uint8_t buffer[BUFFER_SIZE];

size_t buffer_start = 0;
size_t buffer_end = 0;
uint32_t last_mesasge_time = 0;

void buffer_put(uint8_t data)
{
    buffer[buffer_end] = data;
    buffer_end = (buffer_end + 1) % BUFFER_SIZE;
    if (buffer_end == buffer_start)
    {
        DBG_PRINTLN(0, "BUFFER OVERFLOW");
        halt(HALT_ERROR::BUFFER_OVERFLOW);
        return;
    }
}

uint8_t buffer_peek()
{
    if (buffer_start == buffer_end)
    {
        halt(HALT_ERROR::BUFFER_UNDEFLOW);
        return 0;
    }
    return buffer[buffer_start];
}

uint8_t buffer_pop()
{
    uint8_t data = buffer_peek();
    buffer_start = (buffer_start + 1) % BUFFER_SIZE;
    return data;
}

size_t buffer_available()
{
    if (buffer_end >= buffer_start)
    {
        return buffer_end - buffer_start;
    }
    else
    {
        return BUFFER_SIZE - buffer_start + buffer_end;
    }
}

void process_command()
{
    DBG_PRINTLN(2, "Process single command");
    while (buffer_available() && buffer_peek() != COMMAND_START_BYTE)
    {
        uint8_t invalid_popped_byte = buffer_pop();
        DBG_PRINTVAR(2, invalid_popped_byte);
    }

    if (buffer_available() >= COMMAND_SIZE && buffer_peek() == COMMAND_START_BYTE)
    {
        uint8_t params[COMMAND_SIZE];
        uint8_t generated_checksum = 0;
        for (int i = 0; i < COMMAND_SIZE; i++)
        {
            params[i] = buffer_pop();
            generated_checksum ^= params[i];
        }

        uint8_t &start_byte = params[0];
        uint8_t &end_byte = params[COMMAND_SIZE - 1];

        DBG_PRINTVAR(2, start_byte);
        DBG_PRINTVAR(2, end_byte);

        if (start_byte != COMMAND_START_BYTE)
        {
            DBG_PRINTLN(1, "Invalid start byte");
            return;
        }

        if (end_byte != COMMAND_END_BYTE)
        {
            DBG_PRINTLN(1, "Invalid end bye");
            return;
        }

        uint8_t command_id = params[1];

        DBG_PRINTVAR(2, command_id);

        uint8_t checksum = params[COMMAND_SIZE - 2];

        DBG_PRINTVAR(2, checksum);

        generated_checksum ^= end_byte;

        DBG_PRINTVAR(2, checksum);
        DBG_PRINTVAR(2, generated_checksum);

        if (generated_checksum != 0)
        {
            DBG_PRINTLN(1, "Invalid generated checksum");
            return;
        }

        last_mesasge_time = millis();
        pullup_BLUE();

        commands[command_id](&(params[2]));
    }
    else
    {
        DBG_PRINTLN(2, "Not enough data available");
        DBG_PRINTVAR(2, buffer_available());
    }
    DBG_PRINTLN(2, "Done processing single command");
}

void process_commands(HardwareSerial &serial)
{
    wdt_reset();
    uint32_t time = millis();
    if (time>last_mesasge_time && (time-last_mesasge_time)>250)
    {
        pulldown_BLUE();
        DBG_PRINTLN(1, "Stopping motors by expiry");
        stop_motors();
    }
    else if(time<last_mesasge_time)
    {
        last_mesasge_time = time;
    }

    DBG_PRINTLN(2, "Receiving command data");
    while (serial.available())
    {
        buffer_put(serial.read());
    }
    DBG_PRINTLN(2, "Done receiving command data, processing commands");
    while (buffer_available() >= COMMAND_SIZE)
    {
        process_command();
    }
    DBG_PRINTLN(2, "Done processing commands");
}

void init_commands()
{
}
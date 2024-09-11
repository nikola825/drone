//
// Created by nidzo on 6/10/24.
//
//
// Created by nidzo on 6/10/24.
//
#include "common.h"
#include "motors.h"

bool watchdog_enabled;
unsigned long last_recv;
bool halted;

MAKE_PIN(RED, A, 4, 1);
MAKE_PIN(YELLOW, A, 5, 1);
MAKE_PIN(Sending, A, 0, 0);
MAKE_PIN(Receiving, A, 1, 1);
MAKE_PIN(DoneSending, A, 2, 0);
MAKE_PIN(DoneReceiving, A, 3, 1);

#define MP0 L
#define MP1 D
#define MP2 C
#define MP3 F
#define MP4 K

void pullup_ERROR()
{
    pullup_RED();
}

void pulldown_ERROR()
{
    pulldown_RED();
}

void init_pins()
{
    DDR(A) = 0xfa;
    PORTA |= 0x0f;

    DDR(MP0) = 0x00;
    DDR(MP1) = 0x00;
    DDR(MP2) = 0x00;
    DDR(MP3) = 0x00;
    DDR(MP4) = 0x00;
}

void setup()
{
    watchdog_enabled = false;
    halted = true;
    Serial.begin(9600);
    //Serial.setTimeout(2);
    //Serial.println("AAAAAAAAAAAAAA");
    DBG_PRINTLN(1, "Driver MCU Init");
    init_pins();
    init_motors();
    last_recv = millis();
    pullup_RED();
    pulldown_YELLOW();
}

void test_receive_timeout()
{
    wdt_reset();
    auto time = millis();
    if (time < last_recv)
    {
        last_recv = time;
        return;
    }

    if (time - last_recv > INTER_MCU_COMM_TIMEOUT)
    {
        DBG_PRINTLN(1, "Comms expired, stopping motors");
        halted = true;
        stop_motors();
        pullup_RED();
        DBG_PRINTLN(1, "Stopped motors");
        wdt_reset();
    }
}

bool do_recv(MCUPacket &packet)
{
    uint8_t *packet_packed = (uint8_t*)(&packet);
    if (!read_Sending())
    {
        DBG_PRINTLN(2, "Receiving packet");
        pullup_DoneReceiving();
        pulldown_Receiving();

        packet_packed[0] = PIN(MP0);
        packet_packed[1] = PIN(MP1);
        packet_packed[2] = PIN(MP2);
        packet_packed[3] = PIN(MP3);
        packet_packed[4] = PIN(MP4);

        while(read_DoneSending());
        pullup_Receiving();
        pulldown_DoneReceiving();

        last_recv = millis();
        pulldown_RED();
        DBG_PRINTLN(2, "Received packet");
        return true;
    }
    else
    {
        DBG_PRINTLN(2, "No packet");
        return false;
    }
}

/*uint16_t x=0;
void dshot_bitbang_motor0(uint16_t);
void dshot_bitbang_motor1(uint16_t);
void dshot_bitbang_motor2(uint16_t);
void dshot_bitbang_motor3(uint16_t);*/
void loop()
{
    /*int q = Serial.parseInt();
    if(q>0)
    {
        q=q-1;
        Serial.println("GOT ");
        Serial.println(q);
        if(q==42)
        {
            Serial.println("DIR1");
            for(int i=0;i<20;i++)
            {
                dshot_bitbang_motor3(7);
                delay(2);
            }
            for(int i=0;i<20;i++)
            {
                dshot_bitbang_motor3(12);
                delay(4);
            }
            x=0;
        }
        else if (q==43)
        {
            Serial.println("DIR2");
            for(int i=0;i<20;i++)
            {
                dshot_bitbang_motor3(8);
                delay(2);
            }
            for(int i=0;i<20;i++)
            {
                dshot_bitbang_motor3(12);
                delay(4);
            }
            x=0;
        }
        else
        {
            x = q;
        }
    }
    dshot_bitbang_motor0(x);*/
    MCUPacket packet;
    if(halted)
    {
        if (do_recv(packet))
        {
            if (packet.motor0 == 0 && packet.motor1 ==0 && packet.motor2 ==0 && packet.motor3 == 0)
            {
                halted = false;
                init_motors();
                pullup_YELLOW();
                pulldown_RED();

                wdt_enable(WATCHDOG_TIMEOUT);
                watchdog_enabled = true;
                wdt_reset();
            }
        }
        else if (watchdog_enabled)
        {
            packet.motor0 = packet.motor1 =packet.motor2 = packet.motor3 = 0;
            drive(packet);
            wdt_reset();
        }
    }
    else
    {
        if(do_recv(packet))
        {
            DBG_PRINTLN(2, "GOT PACKET");
            DBG_PRINTLN(1, packet.motor0);
            DBG_PRINTLN(2, packet.motor1);
            DBG_PRINTLN(2, packet.motor2);
            DBG_PRINTLN(2, packet.motor3);
            DBG_PRINTLN(2, "GOTTEN");
        }
        drive(packet);
        test_receive_timeout();
    }
}
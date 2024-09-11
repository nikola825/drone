//
// Created by nidzo on 11/2/23.
//

#ifndef CODE_ATMELPORTS_H
#define CODE_ATMELPORTS_H

#include <Arduino.h>

#ifndef DDRC

extern uint8_t dummyport;

#define PORTA dummyport
#define PORTB dummyport
#define PORTC dummyport
#define PORTD dummyport
#define PORTE dummyport
#define PORTF dummyport
#define PORTG dummyport
#define PORTH dummyport
#define PORTI dummyport
#define PORTJ dummyport
#define PORTK dummyport
#define PORTL dummyport

#define PINA dummyport
#define PINB dummyport
#define PINC dummyport
#define PIND dummyport
#define PINE dummyport
#define PINF dummyport
#define PING dummyport
#define PINH dummyport
#define PINI dummyport
#define PINJ dummyport
#define PINK dummyport
#define PINL dummyport

#define DDRA dummyport
#define DDRB dummyport
#define DDRC dummyport
#define DDRD dummyport
#define DDRE dummyport
#define DDRF dummyport
#define DDRG dummyport
#define DDRH dummyport
#define DDRI dummyport
#define DDRJ dummyport
#define DDRK dummyport
#define DDRL dummyport

#endif

#define PIN_FIND(X) PIN##X
#define DDR_FIND(X) DDR##X
#define PORT_FIND(X) PORT##X

#define PIN(X) PIN_FIND(X)
#define PORT(X) PORT_FIND(X)
#define DDR(X) DDR_FIND(X)

#define MAKE_PIN(NAME, PRT, NUMBER, SETALLOW)  \
    inline void  pullup_##NAME()                  \
    {                                             \
         PORT_FIND(PRT) |= ((SETALLOW)<<NUMBER);  \
    }                                             \
    inline void pulldown_##NAME()                 \
    {                                             \
        PORT_FIND(PRT) &= ~((SETALLOW)<<NUMBER);  \
    }                                             \
    inline uint8_t read_##NAME()                  \
    {                                             \
        return PIN_FIND(PRT) & (1<<NUMBER)   ;   \
    }

#endif //CODE_ATMELPORTS_H
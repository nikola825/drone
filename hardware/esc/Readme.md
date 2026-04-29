# ESC

This folder contains PCBs for a 1x and a 4x ESC.
Mostly inspired by the [AM32 reference hardware](https://github.com/am32-firmware/am32-wiki/tree/ad24a0f2b645dd5e00588bb1a56035153b24e888/docs/development)

AM32REF_F051 target can be flashed using standard AM32 flashing process.

## esc1x
Contains one PCB for a single  ESC with solder points for power, control signal and ST-Link connections.

## esc4x 
Contains two PCBs - a logic board and a power board.

The boards are supposed to be soldered together using the castellated edges on the logic board and pads on the power board.

#### Logic board
This PCB contains the microcontrollers and gate drivers for the 4x esc.

It has no high power requirements. Can be fabricated as a cheaper 1 Oz/ft^2 thickness PCB

#### Power board
This PCB contains the mosfets, capacitors, regulators and connections for external stuff.

It is supposed to handle high currents of the motors and should be made with thicker 2 Oz/ft^2 copper.

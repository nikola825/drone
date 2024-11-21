## 3D printable FPV quadcopter frame, SMT32-based flight controller and flight controller code

Hobby project, a homemade FPV drone with custom flight controller and frame.

### Current version - master branch
Folder contents (see readmes for more details):
- `3dp` - Freecad projects with 3D printable drone parts
- `code` - code for the flight controller
- `customboard` - KiCad project for flight controller PCB

Off the shelf parts you'll need:
- screws and screw related parts for the frame - see the readme in `3dp`
- motors and propellers
- ESC
- ELRS receiver
- Camera and VTX if you want FPV video

I couldn't be bothered to do in-detail docs, feel free to email me at (reverse the string to get my email) `moc.liamg@528alokin`

Do not use for anything serious, this was made by a man who started with zero knowledge of electronics, embedded development, rust development, flight controllers and 3D print design.
If you need something reliable, just get off-the-shelf stuff.

### Older versions
- `droneV1` branch contains the old Arduino-as-a-controller + MPU6050 version
- `droneV2` branch contains the old ATMEGA2560 flight controller board with an onboard MPU6050
- `droneV3` branch contains the work-in-progress version of the STM32F411 flight controller

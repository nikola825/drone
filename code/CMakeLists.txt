cmake_minimum_required(VERSION 3.25)
project(code)

set(CMAKE_CXX_STANDARD 17)

set(ARDUINO_CLI arduino-cli)
set(ARDUINO_CFLAGS)
set(ARDUINO_DEVICE /dev/arduino/dron)
set(ARDUINO_FQBN arduino:avr:mega)
set(PY python3)


set(PYTHON_CODE_FOLDER ${PROJECT_SOURCE_DIR}/pycontrol)
set(ARDUINO_CODE_FOLDER ${PROJECT_SOURCE_DIR}/dronecode)
set(ARDUINO_BINARY_FOLDER ${PROJECT_BINARY_DIR}/ardbuild)
FILE(GLOB ARDUINO_CODE ${ARDUINO_CODE_FOLDER}/*.cpp)
FILE(GLOB ARDUINO_CODE_INO ${ARDUINO_CODE_FOLDER}/*.ino)
FILE(GLOB ARDUINO_HEADERS ${ARDUINO_CODE_FOLDER}/*.h)
set(ARDUINO_INCLUDES
        ~/.arduino15/packages/arduino/tools/avr-gcc/7.3.0-atmel3.6.1-arduino7/avr/include/
        ~/.arduino15/packages/arduino/hardware/avr/1.8.5/cores/arduino
        ~/.arduino15/packages/arduino/hardware/avr/1.8.5/variants/mega
        ~/.arduino15/packages/arduino/hardware/avr/1.8.5/libraries/EEPROM/src/
        ~/.arduino15/packages/arduino/hardware/avr/1.8.5/libraries/Wire/src
        ~/Arduino/libraries/Servo/src
        ~/Arduino/libraries/MPU6050/src/
        ~/Arduino/libraries/Adafruit_AHRS/src/)
add_executable(arduino_dummy_target ${ARDUINO_CODE} ${ARDUINO_HEADERS})
target_compile_definitions(arduino_dummy_target PUBLIC UBRRH)
target_compile_definitions(arduino_dummy_target PUBLIC UBRR1H)
target_compile_definitions(arduino_dummy_target PUBLIC UBRR2H)
target_compile_definitions(arduino_dummy_target PUBLIC UBRR3H)
target_include_directories(arduino_dummy_target PUBLIC ${ARDUINO_INCLUDES})

add_custom_command(
        OUTPUT ${ARDUINO_BINARY_FOLDER}
        DEPENDS ${ARDUINO_HEADERS} ${ARDUINO_CODE} ${ARDUINO_CODE_INO}
        COMMAND ${ARDUINO_CLI} compile --output-dir ${ARDUINO_BINARY_FOLDER} --fqbn ${ARDUINO_FQBN} ${ARDUINO_CODE_FOLDER})

add_custom_command(
        OUTPUT ${PYTHON_CODE_FOLDER}/commands_gen.py
        OUTPUT ${PYTHON_CODE_FOLDER}/storage_gen.py
        DEPENDS ${ARDUINO_CODE_FOLDER}/commandimpl.cpp
        DEPENDS ${ARDUINO_CODE_FOLDER}/storage.h
        DEPENDS ${PYTHON_CODE_FOLDER}/generate_commands.py
        COMMAND ${PY} ${PYTHON_CODE_FOLDER}/generate_commands.py ${ARDUINO_CODE_FOLDER}/commandimpl.cpp ${PYTHON_CODE_FOLDER}/commands_gen.py ${ARDUINO_CODE_FOLDER}/storage.h ${PYTHON_CODE_FOLDER}/storage_gen.py
)

add_custom_target(ArduinoBuild
        DEPENDS ${ARDUINO_BINARY_FOLDER}
        DEPENDS ${PYTHON_CODE_FOLDER}/commands_gen.py
        DEPENDS ${PYTHON_CODE_FOLDER}/storage_gen.py)

add_custom_target(ArduinoDeploy
        DEPENDS ${ARDUINO_BINARY_FOLDER}
        DEPENDS ${PYTHON_CODE_FOLDER}/commands_gen.py
        DEPENDS ${PYTHON_CODE_FOLDER}/storage_gen.py
        COMMAND ${ARDUINO_CLI} upload -p ${ARDUINO_DEVICE} --input-dir ${ARDUINO_BINARY_FOLDER} --fqbn ${ARDUINO_FQBN})

set_property(
        TARGET ArduinoDeploy
        APPEND
        PROPERTY ADDITIONAL_CLEAN_FILES ${ARDUINO_BINARY_FOLDER})



add_custom_target(PythonReady
        DEPENDS ${PYTHON_CODE_FOLDER}/commands_gen.py)
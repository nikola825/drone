cmake_minimum_required(VERSION 3.25)
project(code)

set(CMAKE_CXX_STANDARD 17)

set(ARDUINO_CLI arduino-cli)
set(ARDUINO_CFLAGS)
set(ARDUINO_DEVICE_DRIVER /dev/arduino/drone_driver)
set(ARDUINO_DEVICE_CONTROLLER /dev/arduino/drone_driver)
set(ARDUINO_FQBN arduino:avr:mega)
set(PY python3)

set(PYTHON_CODE_FOLDER ${PROJECT_SOURCE_DIR}/pycontrol)
set(DRONE_CODE_FOLDER ${PROJECT_SOURCE_DIR}/dronecode)
set(CONTROLLER_CODE_FOLDER ${DRONE_CODE_FOLDER}/controller)
set(DRIVER_CODE_FOLDER ${DRONE_CODE_FOLDER}/driver)
set(CONTROLLER_BINARY_FOLDER ${PROJECT_BINARY_DIR}/ardbuild/controller)
set(DRIVER_BINARY_FOLDER ${PROJECT_BINARY_DIR}/ardbuild/driver)

FILE(GLOB CONTROLLER_CODE ${CONTROLLER_CODE_FOLDER}/*.cpp)
FILE(GLOB CONTROLLER_CODE_INO ${CONTROLLER_CODE_FOLDER}/*.ino)
FILE(GLOB CONTROLLER_HEADERS ${CONTROLLER_CODE_FOLDER}/*.h)

FILE(GLOB DRIVER_CODE ${DRIVER_CODE_FOLDER}/*.cpp)
FILE(GLOB DRIVER_CODE_INO ${DRIVER_CODE_FOLDER}/*.ino)
FILE(GLOB DRIVER_HEADERS ${DRIVER_CODE_FOLDER}/*.h)

FILE(GLOB COMMON_HEADERS ${DRONE_CODE_FOLDER}/common/*.h)

set(ARDUINO_INCLUDES
        ~/.arduino15/packages/arduino/tools/avr-gcc/7.3.0-atmel3.6.1-arduino7/avr/include/
        ~/.arduino15/packages/arduino/hardware/avr/1.8.5/cores/arduino
        ~/.arduino15/packages/arduino/hardware/avr/1.8.5/variants/mega
        ~/.arduino15/packages/arduino/hardware/avr/1.8.5/libraries/EEPROM/src/
        ~/.arduino15/packages/arduino/hardware/avr/1.8.5/libraries/Wire/src
        ~/Arduino/libraries/Servo/src
        ~/Arduino/libraries/MPU6050/src/
        ~/Arduino/libraries/Adafruit_AHRS/src/
        ${DRONE_CODE_FOLDER}/common)

add_executable(arduino_dummy_target ${DRIVER_CODE} ${DRIVER_HEADERS} ${DRIVER_CODE_INO} ${CONTROLLER_CODE} ${CONTROLLER_CODE_INO} ${CONTROLLER_HEADERS} ${COMMON_HEADERS})

target_compile_definitions(arduino_dummy_target PUBLIC UBRRH)
target_compile_definitions(arduino_dummy_target PUBLIC UBRR1H)
target_compile_definitions(arduino_dummy_target PUBLIC UBRR2H)
target_compile_definitions(arduino_dummy_target PUBLIC UBRR3H)
target_include_directories(arduino_dummy_target PUBLIC ${ARDUINO_INCLUDES})

add_custom_command(
        OUTPUT ${CONTROLLER_BINARY_FOLDER}
        DEPENDS ${ARDUINO_HEADERS} ${CONTROLLER_CODE} ${CONTROLLER_CODE_INO} ${COMMON_HEADERS} ${CONTROLLER_HEADERS}
        COMMAND ${ARDUINO_CLI} compile --output-dir ${CONTROLLER_BINARY_FOLDER} --fqbn ${ARDUINO_FQBN} ${CONTROLLER_CODE_FOLDER} --library ${DRONE_CODE_FOLDER}/common)

add_custom_command(
        OUTPUT ${DRIVER_BINARY_FOLDER}
        DEPENDS ${ARDUINO_HEADERS} ${DRIVER_CODE} ${DRIVER_CODE_INO} ${COMMON_HEADERS} ${DRIVER_HEADERS}
        WORKING_DIRECTORY ${DRIVER_CODE_FOLDER}
        COMMAND ${ARDUINO_CLI} compile --output-dir ${DRIVER_BINARY_FOLDER} --fqbn ${ARDUINO_FQBN} ${DRIVER_CODE_FOLDER} --library ${DRONE_CODE_FOLDER}/common)

add_custom_command(
        OUTPUT ${PYTHON_CODE_FOLDER}/commands_gen.py
        OUTPUT ${PYTHON_CODE_FOLDER}/storage_gen.py
        DEPENDS ${CONTROLLER_CODE_FOLDER}/commandimpl.cpp
        DEPENDS ${CONTROLLER_CODE_FOLDER}/storage.h
        DEPENDS ${PYTHON_CODE_FOLDER}/generate_commands.py
        COMMAND ${PY} ${PYTHON_CODE_FOLDER}/generate_commands.py ${CONTROLLER_CODE_FOLDER}/commandimpl.cpp ${PYTHON_CODE_FOLDER}/commands_gen.py ${CONTROLLER_CODE_FOLDER}/storage.h ${PYTHON_CODE_FOLDER}/storage_gen.py
)

add_custom_target(ControllerBuild
        DEPENDS ${CONTROLLER_BINARY_FOLDER})
        #DEPENDS ${PYTHON_CODE_FOLDER}/commands_gen.py
        #DEPENDS ${PYTHON_CODE_FOLDER}/storage_gen.py)

add_custom_target(ControllerDeploy
        DEPENDS ${CONTROLLER_BINARY_FOLDER}
        #DEPENDS ${PYTHON_CODE_FOLDER}/commands_gen.py
        #DEPENDS ${PYTHON_CODE_FOLDER}/storage_gen.py
        COMMAND ${ARDUINO_CLI} upload -p ${ARDUINO_DEVICE_CONTROLLER} --input-dir ${CONTROLLER_BINARY_FOLDER} --fqbn ${ARDUINO_FQBN})

add_custom_target(DriverBuild
        DEPENDS ${DRIVER_BINARY_FOLDER})

add_custom_target(DriverDeploy
        DEPENDS ${DRIVER_BINARY_FOLDER}
        COMMAND ${ARDUINO_CLI} upload -p ${ARDUINO_DEVICE_DRIVER} --input-dir ${DRIVER_BINARY_FOLDER} --fqbn ${ARDUINO_FQBN})

set_property(
        TARGET ControllerDeploy
        APPEND
        PROPERTY ADDITIONAL_CLEAN_FILES ${CONTROLLER_BINARY_FOLDER})



add_custom_target(PythonReady
        DEPENDS ${PYTHON_CODE_FOLDER}/commands_gen.py)
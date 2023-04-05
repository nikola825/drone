CLI := arduino-cli
DEV := /dev/arduino/mega_55130303335351F0A2F1
FQBN := arduino:avr:mega
ASM := xa
CFLAGS := #--optimize-for-debug

.PHONY: all clean deploy compile

all: 
	echo "make all disabled"

./build/dronecode: dronecode.ino *.cpp *.h
	$(CLI) compile $(CFLAGS) --output-dir  $@ --fqbn $(FQBN) .

compile: ./build/dronecode
	

deploy: ./build/dronecode
	$(CLI) upload -p $(DEV) --input-dir $^ --fqbn $(FQBN)

monitor:
	arduino-cli monitor -p $(DEV)

clean:
	rm -rf ./build/dronecode

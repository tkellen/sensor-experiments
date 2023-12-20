all: build deploy

build:
	cargo objcopy --release -- -O ihex sensors.hex

deploy:
	teensy_loader_cli --mcu=TEENSY41 -vw sensors.hex

shell:
	sudo picocom /dev/ttyACM0
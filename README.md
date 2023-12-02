# si70xx Library
This library is written in rust and can be used to measure the relative temperature and humidity from the si70xx sensor boards from Silicon Labs.<br>

Table of commands:
Command Description                                     Command Code        Function Implemented?
Measure Relative Humidity, Hold Master Mode             0xE5                yes
Measure Relative Humidity, No Hold Master Mode          0xF5                yes
Measure Temperature, Hold Master Mode                   0xE3                yes
Measure Temperature, No Hold Master Mode                0xF3                yes
Read Temperature Value from Previous RH Measurement     0xE0                yes
Reset                                                   0xFE                yes
Write RH/T User Register 1                              0xE6                no
Read RH/T User Register 1                               0xE7                no
Write Heater Control Register                           0x51                no
Read Heater Control Register                            0x11                no
Read Electronic ID 1st Byte                             0xFA 0x0F           yes
Read Electronic ID 2nd Byte                             0xFC 0xC9           yes
Read Firmware Revision                                  0x84 0xB8           yes

# Testing
I only own the Si7021-A20 as of writing this.  It would be good to get the other sensors
to test this library with.


# Sample Output from using this lib in stm32-rs application
stm32-rs application which displays relative temp/humidity

Device FW version: 2.0
Device ID (snb_3): Si7021
RAW sensor ID: 0x3D891CCC15FFB5FF
Temperature: 10.37376 Celcius 50.672768 Fahrenheit
% Relative Humidity: 34.683746 % RH

Temperature: 10.33086 Celcius 50.59555 Fahrenheit
% Relative Humidity: 34.668488 % RH

Temperature: 10.30941 Celcius 50.55694 Fahrenheit
% Relative Humidity: 34.668488 % RH

